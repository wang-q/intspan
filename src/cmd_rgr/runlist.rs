use clap::*;
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("runlist")
        .about("Filter .rg and .tsv files by comparison with a runlist file")
        .after_help(
            r###"
* Lines without a valid range will not be output

Example:

    rgr runlist tests/rgr/intergenic.yml tests/rgr/S288c.rg --op overlap

    rgr runlist tests/rgr/intergenic.yml tests/rgr/ctg.range.tsv --op overlap -H -f 3

"###,
        )
        .arg(
            Arg::new("runlist")
                .help("Set the runlist file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("infiles")
                .help("Set the input files to use")
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
            Arg::new("op")
                .long("op")
                .takes_value(true)
                .default_value("overlap")
                .forbid_empty_values(true)
                .help("operations: overlap, non-overlap or superset"),
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

    let op = args.value_of("op").unwrap();

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
    let yaml = read_yaml(args.value_of("runlist").unwrap());
    let set = yaml2set(&yaml);

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for (i, line) in reader.lines().filter_map(|r| r.ok()).enumerate() {
            if is_header && i == 0 {
                writer.write_fmt(format_args!("{}\n", line))?;
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

            let chr = range.chr();
            let mut intspan = IntSpan::new();
            intspan.add_pair(*range.start(), *range.end());

            //----------------------------
            // Output
            //----------------------------
            match op {
                "overlap" => {
                    if set.contains_key(chr)
                        && !set.get(chr).unwrap().intersect(&intspan).is_empty()
                    {
                        writer.write_fmt(format_args!("{}\n", line))?;
                    }
                }
                "non-overlap" => {
                    if set.contains_key(chr) {
                        if set.get(chr).unwrap().intersect(&intspan).is_empty() {
                            writer.write_fmt(format_args!("{}\n", line))?;
                        }
                    } else {
                        writer.write_fmt(format_args!("{}\n", line))?;
                    }
                }
                "superset" => {
                    if set.contains_key(chr) && set.get(chr).unwrap().superset(&intspan) {
                        writer.write_fmt(format_args!("{}\n", line))?;
                    }
                }
                _ => panic!("Invalid Op"),
            };
        }
    }

    Ok(())
}
