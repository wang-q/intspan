use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("some")
        .about("Extract some records from a runlist json file")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("list")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Loading
    //----------------------------
    let json: BTreeMap<String, Value> = read_json(args.get_one::<String>("infile").unwrap());

    let mut names: BTreeSet<String> = BTreeSet::new();
    for line in read_lines(args.get_one::<String>("list").unwrap()) {
        names.insert(line);
    }

    //----------------------------
    // Operating
    //----------------------------
    let mut out_json: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in &json {
        if names.contains(key) {
            out_json.insert(key.into(), value.clone());
        }
    }

    //----------------------------
    // Output
    //----------------------------
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
