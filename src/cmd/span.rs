use clap::*;
use intspan::*;
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
    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile").unwrap());
    let is_multi: bool = yaml.values().next().unwrap().is_mapping();
    let set_of = yaml2set_m(&yaml);

    let op = args.value_of("op").unwrap();
    let number: i32 = value_t!(args.value_of("number"), i32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --number\n{}", e);
        std::process::exit(1)
    });

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
    let out_yaml = if is_multi {
        set2yaml_m(&res_of)
    } else {
        set2yaml(&res_of.get("__single").unwrap())
    };
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
