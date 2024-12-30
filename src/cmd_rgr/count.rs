use clap::*;
use rust_lapper::{Interval, Lapper};
use std::collections::BTreeMap;
use std::io::BufRead;

// Interval: represent a range from [start, stop), carrying val
type Iv = Interval<u32, u32>; // the first type should be Unsigned

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("count")
        .about("Count overlaps between ranges in a target file and other range files")
        .after_help(
            r###"
* Lines without a valid range will not be output

Example:

    # Count overlaps between two .rg files
    rgr count tests/rgr/S288c.rg tests/rgr/S288c.rg

    # Count overlaps in a .tsv file with headers
    rgr count tests/rgr/ctg.range.tsv tests/rgr/S288c.rg -H -f 3

    # For large .rg files, pre-sorting may improve perfermonce.
    cat *.rg | rgr sort stdin | rgr count target.rg stdin

"###,
        )
        .arg(
            Arg::new("target")
                .required(true)
                .index(1)
                .num_args(1)
                .help("Target .rg/.tsv file"),
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .index(2)
                .num_args(1..)
                .help("Input .rg files to count overlaps with"),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .short('H')
                .action(ArgAction::SetTrue)
                .help("Treat the first line of each file as a header"),
        )
        .arg(
            Arg::new("sharp")
                .long("sharp")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Include lines starting with `#` without changes (default: ignore them)"),
        )
        .arg(
            Arg::new("field")
                .long("field")
                .short('f')
                .value_parser(value_parser!(usize))
                .num_args(1)
                .help("Index of the range field. If not set, the first valid range will be used"),
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
    // Options
    //----------------------------
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());

    let is_sharp = args.get_flag("sharp");
    let is_header = args.get_flag("header");

    let opt_idx_range = args.get_one::<usize>("field").copied().unwrap_or(0);

    //----------------------------
    // Loading
    //----------------------------
    // seq_name => Vector of Intervals
    let mut iv_of: BTreeMap<String, Vec<Iv>> = BTreeMap::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        for line in reader.lines().map_while(Result::ok) {
            if line.starts_with('#') {
                continue;
            }

            let range = intspan::Range::from_str(&line);
            if !range.is_valid() {
                continue;
            }

            let iv = Iv {
                start: *range.start() as u32,
                stop: *range.end() as u32 + 1,
                val: 0,
            };
            let chr = range.chr();
            iv_of.entry(chr.to_string()).or_default().push(iv);
        }
    }

    // seq_name => Lapper
    let mut lapper_of = BTreeMap::new();
    for (chr, ivs) in iv_of {
        let lapper = Lapper::new(ivs);
        lapper_of.insert(chr, lapper);
    }

    //----------------------------
    // Operating
    //----------------------------
    let reader = intspan::reader(args.get_one::<String>("target").unwrap());
    'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
        // Handle the header line
        if is_header && i == 0 {
            writer.write_fmt(format_args!("{}\t{}\n", line, "count"))?;
            continue 'LINE;
        }

        // Handle lines starting with '#'
        if line.starts_with('#') {
            if is_sharp {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
            continue 'LINE;
        }

        let rg = match intspan::extract_rg(&line, opt_idx_range) {
            // Extract the range
            Some(range) => range,
            // Skip lines without a valid range
            None => continue 'LINE,
        };

        let mut count = 0;
        if lapper_of.contains_key(rg.chr()) {
            let lapper = lapper_of.get(rg.chr()).unwrap();
            count = lapper.count(*rg.start() as u32, *rg.end() as u32 + 1);
        }

        //----------------------------
        // Output
        //----------------------------
        writer.write_all(format!("{}\t{}\n", line, count).as_ref())?;
    }

    Ok(())
}
