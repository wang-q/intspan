use clap::*;
use intspan::*;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("concat")
        .about("Concatenate sequence pieces of the same species")
        .after_help(
            r###"
* <name.lst> is a file with a list of names to keep, one per line
* Orders in the output file will following the ones in <name.lst>
* <infiles> are paths to block fasta files, .fas.gz is supported
* infile == stdin means reading from STDIN

"###,
        )
        .arg(
            Arg::new("name.lst")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Path to name.lst"),
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(2)
                .help("Sets the input files to use"),
        )
        .arg(
            Arg::new("phylip")
                .long("phylip")
                .action(ArgAction::SetTrue)
                .help("Output relaxed phylip instead of fasta"),
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
    let is_phylip = args.get_flag("phylip");

    //----------------------------
    // Load names
    //----------------------------
    let needed = read_first_column(args.get_one::<String>("name.lst").unwrap());

    let mut seq_of: BTreeMap<String, String> = BTreeMap::new();
    for name in &needed {
        // default value
        seq_of.insert(name.to_string(), "".to_string());
    }

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let block_names = block.names;
            let length = block.entries.first().unwrap().seq().len();

            for name in &needed {
                if block_names.contains(name) {
                    for entry in &block.entries {
                        let entry_name = entry.range().name();
                        if entry_name == name {
                            let seq = std::str::from_utf8(entry.seq()).unwrap();
                            seq_of.entry(name.to_string()).and_modify(|e| *e += seq);
                        }
                    }
                } else {
                    // fill absent names with ------
                    seq_of
                        .entry(name.to_string())
                        .and_modify(|e| *e += "-".repeat(length).as_str());
                }
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    if is_phylip {
        let count = needed.len();
        let length = seq_of.first_key_value().unwrap().1.len();
        writer.write_all(format!("{} {}\n", count, length).as_ref())?;
        for (k, v) in &seq_of {
            writer.write_all(format!("{} {}\n", k, v).as_ref())?;
        }
    } else {
        for (k, v) in &seq_of {
            writer.write_all(format!(">{}\n{}\n", k, v).as_ref())?;
        }
    }

    Ok(())
}
