use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::Write;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("join")
        .about("Join multiple block fasta files by a common target")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
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
            Arg::new("name")
                .long("name")
                .num_args(1)
                .help("According to this species. Default is the first one"),
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

    let mut name = if args.contains_id("name") {
        args.get_one::<String>("name").unwrap().to_string()
    } else {
        "".to_string()
    };
    let mut block_of: BTreeMap<String, Vec<FasEntry>> = BTreeMap::new();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            if name.is_empty() {
                name = block.names.first().unwrap().to_string();
            }

            let idx = block.names.iter().position(|x| x == &name);
            if idx.is_none() {
                continue;
            }

            let idx = idx.unwrap();
            let header = block.entries.get(idx).unwrap().range().to_string();

            if !block_of.contains_key(&header) {
                // init
                block_of.insert(header.to_string(), vec![]);

                // entry with the selected name goes first
                block_of
                    .get_mut(&header)
                    .unwrap()
                    .push(block.entries.get(idx).unwrap().clone());
            }

            for entry in &block.entries {
                if entry.range().name() == &name {
                    continue;
                }
                block_of.get_mut(&header).unwrap().push(entry.clone());
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for v in block_of.values() {
        for e in v {
            writer.write_all(e.to_string().as_ref())?;
        }
        // end of a block
        writer.write_all("\n".as_ref())?;
    }

    Ok(())
}
