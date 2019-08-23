use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::BTreeMap;

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
    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile").unwrap());
    let s_of = yaml2set_m(&yaml);
    let chrs = chrs_in_sets(&s_of);

    //----------------------------
    // Operating
    //----------------------------
    let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
    for chr in &chrs {
        res.insert(chr.to_string(), IntSpan::new());
    }

    for name in s_of.keys() {
        let set = s_of.get(name.as_str()).unwrap();
        for chr in set.keys() {
            let cur_runlist = set.get(chr).unwrap().to_string();
            res.entry(chr.to_string())
                .and_modify(|e| e.add_runlist(cur_runlist));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_yaml = set2yaml(&res);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
