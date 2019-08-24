use crate::utils::*;
use clap::*;
use intspan::{Coverage, IntSpan, Range};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::io::BufRead;

// TODO: optional chr.sizes to be passed to Coverage::new()

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("cover")
        .about("Output covers on chromosomes")
        .after_help(
            "\
Like command `combine`, but <infiles> are chromosome ranges

    I:1-100
    I(+):90-150             # Strand will be omitted
    S288c.I(-):190-200      # Species name will be omitted \
            ",
        )
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("coverage")
                .help("minimal coverage")
                .long("coverage")
                .short("c")
                .takes_value(true)
                .default_value("1")
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
    let coverage: i32 = value_t!(args.value_of("coverage"), i32).unwrap_or_else(|e| {
        eprintln!("Need a integer for --coverage\n{}", e);
        std::process::exit(1)
    });

    // seq_name => tier_of => IntSpan
    let mut res: BTreeMap<String, Coverage> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let range = Range::from_str(line);
            if range.start() == &0 {
                continue;
            }
            let chr = range.chr();
            if !res.contains_key(chr) {
                let tiers = Coverage::new(coverage);
                res.insert(chr.clone(), tiers);
            }

            res.entry(chr.to_string())
                .and_modify(|e| e.bump(range.start().clone(), range.end().clone()));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();
    for chr in res.keys() {
        set.insert(chr.to_string(), res.get(chr).unwrap().max_tier());
    }
    let out_yaml = set2yaml(&set);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
