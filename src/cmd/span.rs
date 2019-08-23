use crate::utils::*;
use clap::*;
use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("span")
        .about("Operate spans in a YAML file")
        .after_help(
            "\
List of operations
    cover:  a single span from min to max
    holes:  all the holes in runlist
    trim:   remove N integers from each end of each span of runlist
    pad:    add N integers from each end of each span of runlist
    excise: remove all spans smaller than N
    fill:   fill in all holes smaller than or equals to N \
            ",
        )
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("op")
                .long("op")
                .takes_value(true)
                .default_value("cover")
                .empty_values(false)
                .help("operations: cover, holes, trim, pad, excise or fill"),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .takes_value(true)
                .default_value("0")
                .empty_values(false),
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
    let is_mk: bool = master.values().next().unwrap().is_mapping();
    let s1_of = to_set_of(&master);

    let op = args.value_of("op").unwrap();
    let number: i32 = value_t!(args.value_of("number"), i32).unwrap_or_else(|e| {
        println!("Need a integer for --number\n{}", e);
        std::process::exit(1)
    });

    //----------------------------
    // Operating
    //----------------------------
    let mut op_result_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, s1) in &s1_of {
        let mut set_op: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in s1.keys() {
            let intspan_op = match op {
                "cover" => s1.get(chr).unwrap().cover(),
                "holes" => s1.get(chr).unwrap().holes(),
                "trim" => s1.get(chr).unwrap().trim(number),
                "pad" => s1.get(chr).unwrap().pad(number),
                "excise" => s1.get(chr).unwrap().excise(number),
                "fill" => s1.get(chr).unwrap().fill(number),
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
