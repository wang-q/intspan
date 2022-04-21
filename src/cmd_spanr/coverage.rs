use clap::*;
use intspan::*;
use rust_lapper::{Interval, Lapper};
use std::collections::BTreeMap;
use std::io::BufRead;

// Interval: represent a range from [start, stop), carrying val
type Iv = Interval<u32, u32>; // the first type should be Unsigned

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("coverage")
        .about("Output detailed depths of coverages on chromosomes")
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("coverage")
                .help("minimal coverage")
                .long("coverage")
                .short('c')
                .takes_value(true)
                .default_value("1")
                .forbid_empty_values(true),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    //----------------------------
    // Loading
    //----------------------------
    let coverage: i32 = args.value_of_t("coverage").unwrap_or_else(|e| {
        eprintln!("Need a integer for --coverage\n{}", e);
        std::process::exit(1)
    });

    // seq_name => Vector of Intervals
    let mut res: BTreeMap<String, Vec<Iv>> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let range = Range::from_str(&line);
            if !range.is_valid() {
                continue;
            }
            let chr = range.chr();
            if !res.contains_key(chr) {
                let ivs: Vec<Iv> = vec![];
                res.insert(chr.clone(), ivs);
            }

            let iv = Iv {
                start: *range.start() as u32,
                stop: *range.end() as u32 + 1,
                val: 0,
            };

            res.entry(chr.to_string()).and_modify(|e| e.push(iv));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();
    for chr in res.keys() {
        let lapper = Lapper::new(res.get(chr).unwrap().to_owned());
        let ivs = lapper.depth().collect::<Vec<Interval<u32, u32>>>();

        let mut intspan = IntSpan::new();
        for iv in ivs {
            let depth = iv.val as i32;
            if depth < coverage {
                continue;
            }

            intspan.add_pair(iv.start as i32, iv.stop as i32 - 1);
        }

        set.insert(chr.to_string(), intspan);
    }
    let out_yaml = set2yaml(&set);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;

    Ok(())
}
