use clap::*;
use intspan::*;
use rust_lapper::{Interval, Lapper};
use std::collections::BTreeMap;
use std::io::BufRead;

// Interval: represent a range from [start, stop), carrying val
type Iv = Interval<u32, u32>; // the first type should be Unsigned

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("count")
        .about("Count each range overlapping with other range files")
        .after_help(
            r#"
* Field numbers start with 1
* Lines without a valid range will not be output

Example:

    rgr count tests/rgr/S288c.rg tests/rgr/S288c.rg

    rgr count tests/rgr/ctg.range.tsv tests/rgr/S288c.rg -H -f 3

For large range files, pre-sorting may improve perfermonce.

    cat *.rg | rgr sort stdin | rgr count target.rg stdin

"#,
        )
        .arg(
            Arg::new("range")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("infiles")
                .help("Sets the range files to use")
                .required(true)
                .index(2)
                .min_values(1),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .short('H')
                .takes_value(false)
                .help("Treat the first line of each file as a header"),
        )
        .arg(
            Arg::new("sharp")
                .long("sharp")
                .short('s')
                .takes_value(false)
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
        )
        .arg(
            Arg::new("field")
                .long("field")
                .short('f')
                .takes_value(true)
                .help("Set the index of the range field. When not set, the first valid range will be used"),
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
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //----------------------------
    // Options
    //----------------------------
    let mut writer = writer(args.value_of("outfile").unwrap());

    let is_sharp = args.is_present("sharp");
    let is_header = args.is_present("header");

    let idx_range: usize = if args.is_present("field") {
        args.value_of_t("field").unwrap_or_else(|e| {
            eprintln!("Need an integer for --field\n{}", e);
            std::process::exit(1)
        })
    } else {
        0
    };

    //----------------------------
    // Loading
    //----------------------------
    // seq_name => Vector of Intervals
    let mut iv_of: BTreeMap<String, Vec<Iv>> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
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
            if !iv_of.contains_key(chr.as_str()) {
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

    // seq_name => Lapper
    let mut lapper_of = BTreeMap::new();

    for chr in iv_of.keys() {
        let lapper = Lapper::new(iv_of.get(chr).unwrap().to_owned());
        lapper_of.insert(chr.clone(), lapper);
    }

    //----------------------------
    // Operating
    //----------------------------
    let reader = reader(args.value_of("range").unwrap());
    'LINE: for (i, line) in reader.lines().filter_map(|r| r.ok()).enumerate() {
        if is_header && i == 0 {
            writer.write_fmt(format_args!("{}\t{}\n", line, "count"))?;
            continue 'LINE;
        }

        if line.starts_with('#') {
            if is_sharp {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
            continue 'LINE;
        }

        let parts: Vec<&str> = line.split('\t').collect();

        let mut range = Range::new();
        if idx_range == 0 {
            for part in parts {
                let r = Range::from_str(part);
                if r.is_valid() {
                    range = r;
                    break;
                }
            }
        } else {
            range = Range::from_str(parts.get(idx_range - 1).unwrap());
        }

        if !range.is_valid() {
            continue 'LINE;
        }

        let mut count = 0;
        if lapper_of.contains_key(range.chr()) {
            let lapper = lapper_of.get(range.chr()).unwrap();
            count = lapper.count(*range.start() as u32, *range.end() as u32 + 1);
        }

        //----------------------------
        // Output
        //----------------------------
        writer.write_all(format!("{}\t{}\n", line, count).as_ref())?;
    }

    Ok(())
}
