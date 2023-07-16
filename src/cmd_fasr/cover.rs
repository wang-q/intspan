use clap::*;
use intspan::*;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("cover")
        .about("Output covers on chromosomes")
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
                .help("Only output this species"),
        )
        .arg(
            Arg::new("trim")
                .long("trim")
                .num_args(1)
                .value_parser(value_parser!(i32))
                .default_value("0")
                .help("Trim align borders to avoid overlaps in lastz results"),
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
    let mut res_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    let trim = *args.get_one::<i32>("trim").unwrap();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let block_names = block.names;

            if args.contains_id("name") {
                let name = args.get_one::<String>("name").unwrap();
                if !res_of.contains_key(name) {
                    res_of.insert(name.to_string(), BTreeMap::new());
                }
            } else {
                for name in &block_names {
                    if !res_of.contains_key(name) {
                        res_of.insert(name.to_string(), BTreeMap::new());
                    }
                }
            }

            for entry in &block.entries {
                let range = entry.range();
                if !range.is_valid() {
                    continue;
                }

                if args.contains_id("name") {
                    let name = args.get_one::<String>("name").unwrap();
                    if name != range.name() {
                        continue;
                    }
                }

                let res = res_of.get_mut(entry.range().name()).unwrap();

                if !res.contains_key(entry.range().chr()) {
                    res.insert(entry.range().chr().to_string(), IntSpan::new());
                }

                let intspan = range.intspan().clone().trim(trim);
                res.get_mut(entry.range().chr()).unwrap().merge(&intspan);
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_json = if args.contains_id("name") {
        set2json(res_of.first_key_value().unwrap().1)
    } else {
        set2json_m(&res_of)
    };
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
