use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan;
use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("genome")
        .about("Convert chr.size to runlists")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
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
    let length_of = read_sizes(args.value_of("infile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    let mut runlist_of: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in &length_of {
        let mut set = IntSpan::new();
        set.add_pair(1, *value);
        runlist_of.insert(key.into(), set.to_string().into());
    }

    //----------------------------
    // Output
    //----------------------------
    write_runlist(args.value_of("outfile").unwrap(), &runlist_of);
}