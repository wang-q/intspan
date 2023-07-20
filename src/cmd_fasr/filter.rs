use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("filter")
        .about("Filter blocks, and can also be used as a formatter")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
* infile == stdin means reading from STDIN

* If `--name` is not specified, it defaults to the first one in each block

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
            Arg::new("name")
                .long("name")
                .num_args(1)
                .help("Filter blocks based on this species"),
        )
        .arg(
            Arg::new("ge")
                .long("ge")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .help("The length of the sequence >= this value"),
        )
        .arg(
            Arg::new("le")
                .long("le")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .help("The length of the sequence <= this value"),
        )
        .arg(
            Arg::new("upper")
                .long("upper")
                .action(ArgAction::SetTrue)
                .help("Convert all sequences to upper cases"),
        )
        // .arg(
        //     Arg::new("N")
        //         .long("N")
        //         .action(ArgAction::SetTrue)
        //         .help("Convert IUPAC ambiguous codes to 'N' or 'n'"),
        // )
        .arg(
            Arg::new("dash")
                .long("dash")
                .action(ArgAction::SetTrue)
                .help("Remove dashes '-'"),
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
    let is_upper = args.get_flag("upper");
    // let is_n = args.get_flag("N");
    let is_dash = args.get_flag("dash");

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        'BLOCK: while let Ok(block) = next_fas_block(&mut reader) {
            // --name
            let idx = if args.contains_id("name") {
                let name = args.get_one::<String>("name").unwrap();
                if !block.names.contains(name) {
                    continue 'BLOCK;
                }
                block.names.iter().position(|x| x == name).unwrap()
            } else {
                0
            };

            let idx_seq = block.entries[idx].seq();

            // --ge
            if args.contains_id("ge") {
                let value = *args.get_one::<usize>("ge").unwrap();
                if idx_seq.len() < value {
                    continue 'BLOCK;
                }
            }

            // --le
            if args.contains_id("le") {
                let value = *args.get_one::<usize>("le").unwrap();
                if idx_seq.len() > value {
                    continue 'BLOCK;
                }
            }

            for entry in &block.entries {
                let mut out_seq: Vec<u8> = vec![];

                for char in entry.seq() {
                    if is_dash && *char == b'-' {
                        continue;
                    }
                    out_seq.push(*char);
                }

                let out_seq = if is_upper {
                    out_seq.to_ascii_uppercase()
                } else {
                    out_seq
                };

                //----------------------------
                // Output
                //----------------------------
                let out_entry = FasEntry::from(entry.range(), &out_seq);
                writer.write_all(out_entry.to_string().as_ref())?;
            }

            // end of a block
            writer.write_all("\n".as_ref())?;
        }
    }

    Ok(())
}
