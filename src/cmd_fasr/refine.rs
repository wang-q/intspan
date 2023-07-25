use clap::*;
use crossbeam::channel::bounded;
use intspan::*;
use std::string::String;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("refine")
        .about("Realign files with external programs and trim unwanted regions")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* List of msa:
    * mafft
    * muscle
    * clustalw
    * none: means skip realigning

* For aligned files converted from .axt or .maf, we can use the `--quick` option
  to align only indel adjacent regions

* Running in parallel mode with 1 reader, 1 writer and the corresponding number of workers
    * The order of blocks in output may be different from the original

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("msa")
                .long("msa")
                .num_args(1)
                .default_value("clustalw")
                .help("Aligning program"),
        )
        .arg(
            Arg::new("has_outgroup")
                .long("outgroup")
                .action(ArgAction::SetTrue)
                .help("There are outgroups at the end of each block"),
        )
        .arg(
            Arg::new("chop")
                .long("chop")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("0")
                .help("Chop head and tail indels"),
        )
        .arg(
            Arg::new("is_quick")
                .long("quick")
                .action(ArgAction::SetTrue)
                .help("Quick mode, only aligns indel adjacent regions"),
        )
        .arg(
            Arg::new("pad")
                .long("pad")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("50")
                .help("In quick mode, enlarge indel regions"),
        )
        .arg(
            Arg::new("fill")
                .long("fill")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("50")
                .help("In quick mode, fill holes between indel"),
        )
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .value_parser(value_parser!(usize))
                .num_args(1)
                .default_value("1")
                .help("Running in parallel mode, the number of threads"),
        )
        .arg(
            Arg::new("outfile")
                .long("outfile")
                .short('o')
                .num_args(1)
                .default_value("stdout")
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let parallel = *args.get_one::<usize>("parallel").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    if parallel == 1 {
        let mut writer = writer(args.get_one::<String>("outfile").unwrap());

        for infile in args.get_many::<String>("infiles").unwrap() {
            let mut reader = reader(infile);
            while let Ok(block) = next_fas_block(&mut reader) {
                let out_string = proc_block(&block, args)?;
                writer.write_all(out_string.as_ref())?;
            }
        }
    } else {
        proc_block_p(args)?;
    }

    Ok(())
}

fn proc_block(block: &FasBlock, args: &ArgMatches) -> anyhow::Result<String> {
    //----------------------------
    // Args
    //----------------------------
    let msa = args.get_one::<String>("msa").unwrap();
    let has_outgroup = args.get_flag("has_outgroup");
    let chop = *args.get_one::<usize>("chop").unwrap();
    let is_quick = args.get_flag("is_quick");
    let pad = *args.get_one::<usize>("pad").unwrap();
    let fill = *args.get_one::<usize>("fill").unwrap();

    //----------------------------
    // Realigning
    //----------------------------
    let mut seqs: Vec<String> = vec![];
    let mut ranges = vec![];
    for entry in &block.entries {
        seqs.push(String::from_utf8(entry.seq().to_vec()).unwrap());
        ranges.push(entry.range().clone());
    }

    let mut aligned = vec![];
    if *msa == "none".to_string() {
        for seq in seqs {
            aligned.push(seq.clone());
        }
    } else {
        if is_quick {
            aligned = align_seqs_quick(&seqs, msa, pad as i32, fill as i32)?;
        } else {
            aligned = align_seqs(&seqs, msa)?;
        }
    };

    //----------------------------
    // Trimming
    //----------------------------
    trim_pure_dash(&mut aligned);
    if has_outgroup {
        trim_outgroup(&mut aligned);
        let _ = trim_complex_indel(&mut aligned);
    }

    if chop > 0 {
        trim_head_tail(&mut aligned, &mut ranges, chop);
    }

    //----------------------------
    // Output
    //----------------------------
    let mut out_string = "".to_string();
    for (range, seq) in ranges.iter().zip(aligned) {
        // eprintln!("range = {:#?}", range.to_string());
        out_string += format!(">{}\n{}\n", range, seq).as_ref();
    }

    // end of a block
    out_string += "\n";

    Ok(out_string)
}

// Adopt from https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html#create-a-parallel-pipeline
fn proc_block_p(args: &ArgMatches) -> anyhow::Result<()> {
    let parallel = *args.get_one::<usize>("parallel").unwrap();
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    // Channel 1 - Read files to blocks
    let (snd1, rcv1) = bounded::<FasBlock>(10);
    // Channel 2 - Results
    let (snd2, rcv2) = bounded(10);

    crossbeam::scope(|s| {
        //----------------------------
        // Reader thread
        //----------------------------
        s.spawn(|_| {
            for infile in args.get_many::<String>("infiles").unwrap() {
                let mut reader = reader(infile);
                while let Ok(block) = next_fas_block(&mut reader) {
                    snd1.send(block).unwrap();
                }
            }
            // Close the channel - this is necessary to exit the for-loop in the worker
            drop(snd1);
        });

        //----------------------------
        // Worker threads
        //----------------------------
        for _ in 0..parallel {
            // Send to sink, receive from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // Spawn workers in separate threads
            s.spawn(move |_| {
                // Receive until channel closes
                for block in recvr.iter() {
                    let out_string = proc_block(&block, args).unwrap();
                    sendr.send(out_string).unwrap();
                }
            });
        }
        // Close the channel, otherwise sink will never exit the for-loop
        drop(snd2);

        //----------------------------
        // Writer thread
        //----------------------------
        for out_string in rcv2.iter() {
            writer.write_all(out_string.as_ref()).unwrap();
        }
    })
    .unwrap();

    Ok(())
}
