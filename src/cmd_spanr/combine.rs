use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("combine")
        .about("Combine multiple sets of runlists in a json file")
        .after_help(
            r###"
It's expected that the JSON file contains multiple sets of runlists,
otherwise this command will make no effects

"###,
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .num_args(1)
                .default_value("union")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Operations: intersect, union, diff or xor"),
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
    let json: BTreeMap<String, Value> = read_json(args.get_one::<String>("infile").unwrap());
    let s_of = json2set_m(&json);
    let chrs = chrs_in_sets(&s_of);

    let op = args.get_one::<String>("op").unwrap().as_str();

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
    let out_json = set2json(&res);
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
