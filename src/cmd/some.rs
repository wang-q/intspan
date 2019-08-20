use crate::utils::{
    read_lines, read_runlist, read_sizes, reader, write_lines, write_runlist, writer,
};
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan;
use intspan::IntSpan;
use serde::de::Unexpected::Str;
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

// test command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let master: BTreeMap<String, Value> = read_runlist(args.value_of("infile").unwrap());

    let mut all_names: BTreeSet<String> = BTreeSet::new();
    for line in read_lines(args.value_of("list").unwrap()) {
        all_names.insert(line);
    }

    //----------------------------
    // Operating
    //----------------------------
    let mut out_map: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in &master {
        if all_names.contains(key) {
            out_map.insert(key.into(), value.clone());
        }
    }

    //----------------------------
    // Output
    //----------------------------
    write_runlist(args.value_of("outfile").unwrap(), &out_map);
}
