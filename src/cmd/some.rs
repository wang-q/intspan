use crate::utils::*;
use clap::*;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("some")
        .about("Extract some records from a runlist yaml file")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("list")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .empty_values(false)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile").unwrap());

    let mut names: BTreeSet<String> = BTreeSet::new();
    for line in read_lines(args.value_of("list").unwrap()) {
        names.insert(line);
    }

    //----------------------------
    // Operating
    //----------------------------
    let mut out_yaml: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in &yaml {
        if names.contains(key) {
            out_yaml.insert(key.into(), value.clone());
        }
    }

    //----------------------------
    // Output
    //----------------------------
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
