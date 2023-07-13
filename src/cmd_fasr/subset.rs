use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("subset")
        .about("Extract a subset of species")
        .after_help(
            r###"
* <infile> is path to block fasta file, .fas.gz is supported
* infile == stdin means reading from STDIN
* <name.lst> is a file with a list of names to keep, one per line
* Orders in the output file will following the ones in <name.lst>

"###,
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("name.lst")
                .required(true)
                .num_args(1)
                .index(2)
                .help("Path to name.lst"),
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
    // Loading
    //----------------------------
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());
    let mut reader = reader(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Load names
    //----------------------------
    let needed = read_first_column(args.get_one::<String>("name.lst").unwrap());

    while let Ok(block) = next_fas_block(&mut reader) {
        let block_names = block.names;

        for name in &needed {
            if block_names.contains(name) {
                for entry in &block.entries {
                    let entry_name = entry.range().name();
                    //----------------------------
                    // Output
                    //----------------------------
                    if entry_name == name {
                        writer.write_all(entry.to_string().as_ref())?;
                    }
                }
            }
        }

        // end of a block
        writer.write_all("\n".as_ref())?;
    }

    Ok(())
}
