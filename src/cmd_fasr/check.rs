use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("check")
        .about("Check genome locations in block fasta headers")
        .after_help(
            r###"
* <genome.fa> is a multi-fasta file contains genome sequences

* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* Need `samtools` in $PATH

"###,
        )
        .arg(
            Arg::new("genome.fa")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Path to genome.fa"),
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(2)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("name")
                .long("name")
                .num_args(1)
                .help("Which species to be checked, omit this will check all sequences"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());
    let genome = args.get_one::<String>("genome.fa").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let block_names = block.names;

            if args.contains_id("name") {
                let name = args.get_one::<String>("name").unwrap();
                if block_names.contains(name) {
                    for entry in &block.entries {
                        let entry_name = entry.range().name();
                        //----------------------------
                        // Output
                        //----------------------------
                        if entry_name == name {
                            let status = check_seq(entry, genome)?;
                            writer
                                .write_all(format!("{}\t{}\n", entry.range(), status).as_ref())?;
                        }
                    }
                }
            } else {
                for entry in &block.entries {
                    let status = check_seq(entry, genome)?;
                    writer.write_all(format!("{}\t{}\n", entry.range(), status).as_ref())?;
                }
            }
        }
    }

    Ok(())
}

fn check_seq(entry: &FasEntry, genome: &str) -> anyhow::Result<String> {
    let range = entry.range();
    let seq = if range.strand() == "-" {
        bio::alphabets::dna::revcomp(entry.seq())
    } else {
        entry.seq().to_vec()
    };
    let seq = std::str::from_utf8(&seq)
        .unwrap()
        .to_string()
        .to_ascii_uppercase()
        .replace('-', "");

    let pos = format!("{}:{}-{}", range.chr(), range.start(), range.end());
    let gseq = get_seq_faidx(genome, &pos)?.to_ascii_uppercase();

    let status = if seq == gseq { "OK" } else { "FAILED" };

    Ok(status.to_string())
}
