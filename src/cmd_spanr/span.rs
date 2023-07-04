use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("span")
        .about("Operate spans in a JSON file")
        .after_help(
            r###"
List of operations

* cover:  a single span from min to max
* holes:  all the holes in runlist
* trim:   remove N integers from each end of each span of runlist
* pad:    add N integers from each end of each span of runlist
* excise: remove all spans smaller than N
* fill:   fill in all holes smaller than or equals to N

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
                .default_value("cover")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("operations: cover, holes, trim, pad, excise or fill"),
        )
        .arg(
            Arg::new("number")
                .long("number")
                .short('n')
                .num_args(1)
                .value_parser(value_parser!(i32))
                .default_value("0"),
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
    let is_multi: bool = json.values().next().unwrap().is_object();
    let set_of = json2set_m(&json);

    let op = args.get_one::<String>("op").unwrap().as_str();
    let number = *args.get_one::<i32>("number").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    let mut res_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, set) in &set_of {
        let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in set.keys() {
            let intspan = match op {
                "cover" => set.get(chr).unwrap().cover(),
                "holes" => set.get(chr).unwrap().holes(),
                "trim" => set.get(chr).unwrap().trim(number),
                "pad" => set.get(chr).unwrap().pad(number),
                "excise" => set.get(chr).unwrap().excise(number),
                "fill" => set.get(chr).unwrap().fill(number),
                _ => panic!("Invalid IntSpan Op"),
            };
            //            println!("Op {}: {}", op, op_intspan.to_string());
            res.insert(chr.into(), intspan);
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
