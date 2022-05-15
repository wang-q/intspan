use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
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
                .help("Set the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .short('H')
                .takes_value(false)
                .help("Treat the first line of each file as a header"),
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
    let mut line_map: BTreeMap<String, Range> = BTreeMap::new();
    let mut invalids: Vec<String> = vec![];

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for (i, line) in reader.lines().filter_map(|r| r.ok()).enumerate() {
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

    let mut valids: Vec<String> = line_map.keys().into_iter().map(|e| e.to_string()).collect();
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
