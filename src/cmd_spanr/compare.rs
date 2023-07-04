use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("compare")
        .about("Compare one JSON file against others")
        .after_help("Only the *first* file can contain multiple sets of runlists")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .index(2)
                .min_values(1),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .takes_value(true)
                .default_value("intersect")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Operations: intersect, union, diff or xor"),
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
    // first file
    let json: BTreeMap<String, Value> = read_json(args.get_one::<String>("infile").unwrap());
    let is_multi: bool = json.values().next().unwrap().is_object();
    let mut s1_of = json2set_m(&json);

    // second file or more
    let mut s2s = vec![];

    for infile in args.get_many::<String>("infiles").unwrap() {
        let json_s = read_json(infile);
        let s2 = json2set(&json_s);
        s2s.push(s2);
    }

    let op = args.get_one::<String>("op").unwrap().as_str();

    //----------------------------
    // Operating
    //----------------------------
    // give empty intspan to non-existed chrs
    let mut chrs = chrs_in_sets(&s1_of);
    for s2 in &s2s {
        for chr in s2.keys() {
            chrs.insert(chr.to_string());
        }
    }
    fill_up_m(&mut s1_of, &chrs);

    for s2 in s2s.iter_mut() {
        fill_up_s(s2, &chrs);
    }

    let mut res_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, s1) in &s1_of {
        let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in s1.keys() {
            let mut intspan_op = s1.get(chr).unwrap().copy();
            for s2 in s2s.iter() {
                intspan_op = match op {
                    "intersect" => intspan_op.intersect(s2.get(chr).unwrap()),
                    "diff" => intspan_op.diff(s2.get(chr).unwrap()),
                    "union" => intspan_op.union(s2.get(chr).unwrap()),
                    "xor" => intspan_op.xor(s2.get(chr).unwrap()),
                    _ => panic!("Invalid IntSpan Op"),
                };
                //                eprintln!("Op {}: {}", op, intspan_op.to_string());
            }
            res.insert(chr.into(), intspan_op);
        }
        res_of.insert(name.into(), res);
    }

    //----------------------------
    // Output
    //----------------------------
    let out_json = if is_multi {
        set2json_m(&res_of)
    } else {
        set2json(res_of.get("__single").unwrap())
    };
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
