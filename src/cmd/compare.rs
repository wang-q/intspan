use crate::utils::*;
use clap::*;
use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("compare")
        .about("Compare 2 YAML files")
        .after_help("Only the *first* file can contain multiple sets of runlists")
        .arg(
            Arg::with_name("infile1")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("infile2")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("op")
                .long("op")
                .takes_value(true)
                .default_value("intersect")
                .empty_values(false)
                .help("operations: intersect, union, diff or xor"),
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
    let master: BTreeMap<String, Value> = read_runlist(args.value_of("infile1").unwrap());
    let is_mk: bool = master.values().next().unwrap().is_mapping();
    let mut s1_of = to_set_of(&master);

    let single: BTreeMap<String, Value> = read_runlist(args.value_of("infile2").unwrap());
    let mut s2 = runlist2set(&single);

    let op = args.value_of("op").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    let mut chrs = chrs_in_sets(&s1_of);
    for chr in s2.keys() {
        chrs.insert(chr.to_string());
    }

    fill_up(&mut s1_of, &chrs);
    fill_up_s(&mut s2, &chrs);

    let mut op_result_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, s1) in &s1_of {
        let mut set_op: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in s1.keys() {
            let intspan_op = match op {
                "intersect" => s1.get(chr).unwrap().intersect(s2.get(chr).unwrap()),
                "diff" => s1.get(chr).unwrap().diff(s2.get(chr).unwrap()),
                "union" => s1.get(chr).unwrap().union(s2.get(chr).unwrap()),
                "xor" => s1.get(chr).unwrap().xor(s2.get(chr).unwrap()),
                _ => panic!("Invalid IntSpan Op"),
            };
            //            println!("Op {}: {}", op, op_intspan.to_string());
            set_op.insert(chr.into(), intspan_op);
        }
        op_result_of.insert(name.into(), set_op);
    }

    //----------------------------
    // Output
    //----------------------------
    if is_mk {
        let out_runlist = set2runlist_m(&op_result_of);
        write_runlist(args.value_of("outfile").unwrap(), &out_runlist);
    } else {
        let out_runlist = set2runlist(&op_result_of.get("__single").unwrap());
        write_runlist(args.value_of("outfile").unwrap(), &out_runlist);
    }
}
