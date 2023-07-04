use clap::*;
use intspan::*;
use rust_lapper::{Interval, Lapper};
use std::collections::BTreeMap;
use std::io::BufRead;

// Interval: represent a range from [start, stop), carrying val
type Iv = Interval<u32, u32>; // the first type should be Unsigned

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("coverage")
        .about("Output minimum or detailed depth of coverage on chromosomes")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input file to use"),
        )
        .arg(
            Arg::new("minimum")
                .long("minimum")
                .short('m')
                .value_parser(value_parser!(i32))
                .num_args(1)
                .default_value("1")
                .help("Set the minimum depth of coverage"),
        )
        .arg(
            Arg::new("detailed")
                .long("detailed")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Output detailed depth"),
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
    let minimum = *args.get_one::<i32>("minimum").unwrap();
    let is_detailed = args.get_flag("detailed");

    // seq_name => Vector of Intervals
    let mut iv_of: BTreeMap<String, Vec<Iv>> = BTreeMap::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            if line.starts_with('#') {
                continue;
            }
            let range = Range::from_str(&line);
            if !range.is_valid() {
                continue;
            }
            let chr = range.chr();
            if !iv_of.contains_key(chr) {
                let ivs: Vec<Iv> = vec![];
                iv_of.insert(chr.clone(), ivs);
            }

            let iv = Iv {
                start: *range.start() as u32,
                stop: *range.end() as u32 + 1,
                val: 0,
            };

            iv_of.entry(chr.to_string()).and_modify(|e| e.push(iv));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    if is_detailed {
        // Multi
        let mut set_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();

        for chr in iv_of.keys() {
            let lapper = Lapper::new(iv_of.get(chr).unwrap().to_owned());
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

        let out_json = set2json_m(&set_of);
        write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;
    } else {
        // Single
        // chr => IntSpan
        let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

        for chr in iv_of.keys() {
            let lapper = Lapper::new(iv_of.get(chr).unwrap().to_owned());
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

        let out_json = set2json(&set);
        write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;
    }

    Ok(())
}
