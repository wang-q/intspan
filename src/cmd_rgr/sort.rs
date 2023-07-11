use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("sort")
        .about("Sort .rg and .tsv files by a range field")
        .after_help(
            r###"
* If no part is a valid range, the line will be written to the last

Example:

    rgr sort tests/rgr/S288c.rg

    rgr sort tests/rgr/ctg.range.tsv

    rgr sort tests/rgr/ctg.range.tsv -H -f 3

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input file to use")
        )
        .arg(
            Arg::new("header")
                .long("header")
                .short('H')
                .action(ArgAction::SetTrue)
                .help("Treat the first line of each file as a header"),
        )
        .arg(
            Arg::new("field")
                .long("field")
                .short('f')
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Set the index of the range field. When not set, the first valid range will be used"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    let is_header = args.get_flag("header");

    let idx_range = if args.contains_id("field") {
        *args.get_one::<usize>("field").unwrap()
    } else {
        0
    };

    //----------------------------
    // Loading
    //----------------------------
    let mut line_map: BTreeMap<String, Range> = BTreeMap::new();
    let mut invalids: Vec<String> = vec![];

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            if is_header && i == 0 {
                writer.write_fmt(format_args!("{}\n", line))?;
                continue 'LINE;
            }

            let parts: Vec<&str> = line.split('\t').collect();

            if idx_range == 0 {
                for part in parts {
                    let range = Range::from_str(part);
                    if range.is_valid() {
                        line_map.insert(line.clone(), range);
                        continue 'LINE;
                    }
                }
            } else {
                let range = Range::from_str(parts.get(idx_range - 1).unwrap());
                if range.is_valid() {
                    line_map.insert(line.clone(), range);
                    continue 'LINE;
                }
            }

            invalids.push(line.clone()); // No part is a valid range
        }
    }

    let mut valids: Vec<String> = line_map.keys().map(|e| e.to_string()).collect();
    {
        // by chromosome strand
        valids.sort_by_cached_key(|k| line_map.get(k).unwrap().strand());

        // by start point on chromosomes
        valids.sort_by_cached_key(|k| line_map.get(k).unwrap().start());

        // by chromosome name
        valids.sort_by_cached_key(|k| line_map.get(k).unwrap().chr());
    }

    //----------------------------
    // Output
    //----------------------------
    for line in &valids {
        writer.write_fmt(format_args!("{}\n", line))?;
    }
    for line in &invalids {
        writer.write_fmt(format_args!("{}\n", line))?;
    }

    Ok(())
}
