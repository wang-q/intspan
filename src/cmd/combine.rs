use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan;
use intspan::IntSpan;
use serde::Serialize;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("combine")
        .about("Combine multiple sets of runlists in a yaml file")
        .after_help(
            "It's expected that the YAML file contains multiple sets of runlists, \
             otherwise this command will make no effects",
        )
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

// command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let master: BTreeMap<String, Value> = read_runlist(args.value_of("infile").unwrap());
    let set_of = to_set_of(&master);
    let chrs = chrs_in_sets(&set_of);

    //----------------------------
    // Operating
    //----------------------------
    let mut op_result: BTreeMap<String, IntSpan> = BTreeMap::new();
    for chr in &chrs {
        op_result.insert(chr.to_string(), IntSpan::new());
    }

    for name in set_of.keys() {
        let set = set_of.get(name.as_str()).unwrap();
        for chr in set.keys() {
            let mut intspan = IntSpan::new();
            let cur_result = op_result.get(chr).unwrap();
            //            println!("cur_result {}", cur_result.to_string());

            let cur_intspan = set.get(chr).unwrap();
            //            println!("cur_intspan {}", cur_intspan.to_string());

            intspan.add_runlist(cur_result.to_string());
            intspan.add_runlist(cur_intspan.to_string());

            op_result.insert(chr.clone(), intspan);
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_runlist = set2runlist(&op_result);
    write_runlist(args.value_of("outfile").unwrap(), &out_runlist);
}
