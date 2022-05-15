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
For large range files, pre-sorting may improve perfermonce.

    cat *.rg | linkr sort | spanr count target.ranges stdin

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
            Arg::new("sharp")
                .long("sharp")
                .short('s')
                .takes_value(false)
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
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
    // Loading
    //----------------------------
    let mut writer = writer(args.value_of("outfile").unwrap());

    let is_sharp = args.is_present("sharp");

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
    'line: for line in reader(args.value_of("range").unwrap())
        .lines()
        .filter_map(|r| r.ok())
    {
        if line.starts_with('#') {
            if is_sharp {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
            continue 'line;
        }

        let range = Range::from_str(&line);
        if !range.is_valid() {
            continue 'line;
        }
        let chr = range.chr();

        let mut count = 0;
        if lapper_of.contains_key(chr.as_str()) {
            let lapper = lapper_of.get(chr.as_str()).unwrap();
            count = lapper.count(*range.start() as u32, *range.end() as u32 + 1);
        }

        //----------------------------
        // Output
        //----------------------------
        writer.write_all(format!("{}\t{}\n", line, count).as_ref())?;
    }

    Ok(())
}
