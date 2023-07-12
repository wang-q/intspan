use clap::*;
use intspan::*;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("name")
        .about("Scan block fasta files and output all species names")
        .after_help(
            r###"
* <infiles> are paths to fas files, .fas.gz is supported
* infile == stdin means reading from STDIN

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Also count name occurrences"),
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
    let is_count = args.get_flag("count");

    let mut count_of: BTreeMap<String, i32> = BTreeMap::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            for entry in block.entries {
                let range = entry.range();

                count_of.entry(range.name().to_string())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for (k, v) in &count_of {
        if is_count {
            writer.write_all(format!("{}\t{}\n", k, v).as_ref())?;
        } else {
            writer.write_all(format!("{}\n", k).as_ref())?;
        }
    }

    Ok(())
}
