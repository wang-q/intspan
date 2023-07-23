use clap::*;
use crossbeam::channel::bounded;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("consensus")
        .about("Generate consensus sequences by POA")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* Need `spoa` in $PATH
    * The original `poa` was unstable and sometimes crashed

* Running in parallel mode with 1 reader, 1 writer and the corresponding number of workers
    * The order of output may be different from the original

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
            Arg::new("cname")
                .long("cname")
                .num_args(1)
                .default_value("consensus")
                .help("Consensus name"),
        )
        .arg(
            Arg::new("has_outgroup")
                .long("outgroup")
                .action(ArgAction::SetTrue)
                .help("There are outgroups at the end of each block"),
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
    let cname = args.get_one::<String>("cname").unwrap();
    let has_outgroup = args.get_flag("has_outgroup");

    //----------------------------
    // Operating
    //----------------------------
    let mut seqs = vec![];

    let outgroup = if has_outgroup {
        Some(block.entries.iter().last().unwrap())
    } else {
        None
    };

    for entry in &block.entries {
        seqs.push(entry.seq().as_ref());
    }
    if outgroup.is_some() {
        seqs.pop().unwrap();
    }

    let mut cons = get_consensus_poa(&seqs).unwrap();
    cons = cons.replace('-', "");

    let mut range = block.entries.first().unwrap().range().clone();

    //----------------------------
    // Output
    //----------------------------
    let mut out_string = "".to_string();
    if range.is_valid() {
        *range.name_mut() = cname.to_string();
        out_string += format!(">{}\n{}\n", range, cons).as_ref();
    } else {
        out_string += format!(">{}\n{}\n", cname, cons).as_ref();
    }
    if outgroup.is_some() {
        out_string += outgroup.unwrap().to_string().as_ref();
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
