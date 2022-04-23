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
        .about("Output minimum or detailed depth of coverage on chromosomes")
        .arg(
            Arg::new("infiles")
                .help("Set the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("minimum")
                .help("Set the minimum depth of coverage")
                .long("minimum")
                .short('m')
                .takes_value(true)
                .default_value("1")
                .forbid_empty_values(true),
        )
        .arg(
            Arg::new("detailed")
                .help("Output detailed depth")
                .long("detailed")
                .short('d'),
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
    let minimum: i32 = args.value_of_t("minimum").unwrap_or_else(|e| {
        eprintln!("Need a integer for --minimum\n{}", e);
        std::process::exit(1)
    });
    let is_detailed = args.is_present("detailed");

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
    if is_detailed {
        // Multi
        let mut set_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();

        for chr in res.keys() {
            let lapper = Lapper::new(res.get(chr).unwrap().to_owned());
            let ivs = lapper.depth().collect::<Vec<Interval<u32, u32>>>();

            // depth => IntSpan
            let mut intspan_of: BTreeMap<String, IntSpan> = BTreeMap::new();

            for iv in ivs {
                let depth = iv.val as i32;
                if depth < minimum {
                    continue;
                }

                let depth = format!("{}", depth);

                if !set_of.contains_key(&depth) {
                    set_of.insert(depth.clone(), BTreeMap::new());
                }

                if !intspan_of.contains_key(&depth) {
                    intspan_of.insert(depth.clone(), IntSpan::new());
                }

                intspan_of
                    .entry(depth)
                    .and_modify(|e| e.add_pair(iv.start as i32, iv.stop as i32 - 1));
            }

            for depth in intspan_of.keys() {
                set_of
                    .get_mut(depth)
                    .unwrap()
                    .insert(chr.clone(), intspan_of.get(depth).unwrap().clone());
            }
        }

        let out_yaml = set2yaml_m(&set_of);
        write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;
    } else {
        // Single
        // chr => IntSpan
        let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

        for chr in res.keys() {
            let lapper = Lapper::new(res.get(chr).unwrap().to_owned());
            let ivs = lapper.depth().collect::<Vec<Interval<u32, u32>>>();

            let mut intspan = IntSpan::new();
            for iv in ivs {
                let depth = iv.val as i32;
                if depth < minimum {
                    continue;
                }

                intspan.add_pair(iv.start as i32, iv.stop as i32 - 1);
            }

            set.insert(chr.to_string(), intspan);
        }

        let out_yaml = set2yaml(&set);
        write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;
    }

    Ok(())
}
