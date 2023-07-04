use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("genome")
        .about("Convert chr.size to runlists")
        .arg(
            Arg::new("infile")
                .required(true)
                .index(1)
                .help("Sets the input file to use"),
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
    let sizes = read_sizes(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    let mut json: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in sizes {
        let mut intspan = IntSpan::new();
        intspan.add_pair(1, value);
        json.insert(key, intspan.to_string().into());
    }

    //----------------------------
    // Output
    //----------------------------
    write_json(args.get_one::<String>("outfile").unwrap(), &json)?;

    Ok(())
}
