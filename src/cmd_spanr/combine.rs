use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::*;
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
            Arg::with_name("op")
                .long("op")
                .takes_value(true)
                .default_value("union")
                .empty_values(false)
                .help("Operations: intersect, union, diff or xor"),
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
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    //----------------------------
    // Loading
    //----------------------------
    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile").unwrap());
    let s_of = yaml2set_m(&yaml);
    let chrs = chrs_in_sets(&s_of);

    let op = args.value_of("op").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
    fill_up_s(&mut res, &chrs);

    let names: Vec<_> = s_of.keys().cloned().collect();
    let first = names[0].clone();

    for name in names {
        let set = s_of.get(name.as_str()).unwrap();
        for chr in set.keys() {
            if name == first {
                let intspan = set.get(chr).unwrap();
                res.entry(chr.to_string()).and_modify(|e| e.merge(intspan));
            } else {
                let mut intspan_op = res.get(chr).unwrap().copy();
                intspan_op = match op {
                    "intersect" => intspan_op.intersect(set.get(chr).unwrap()),
                    "diff" => intspan_op.diff(set.get(chr).unwrap()),
                    "union" => intspan_op.union(set.get(chr).unwrap()),
                    "xor" => intspan_op.xor(set.get(chr).unwrap()),
                    _ => panic!("Invalid IntSpan Op"),
                };
                //                eprintln!("Op {}: {}", op, intspan_op.to_string());
                res.insert(chr.into(), intspan_op);
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_yaml = set2yaml(&res);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;

    Ok(())
}
