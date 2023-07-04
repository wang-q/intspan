use clap::*;
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("runlist")
        .about("Filter .rg and .tsv files by comparison with a runlist file")
        .after_help(
            r###"
* Lines without a valid range will not be output

Example:

    rgr runlist tests/rgr/intergenic.json tests/rgr/S288c.rg --op overlap

    rgr runlist tests/rgr/intergenic.json tests/rgr/ctg.range.tsv --op overlap -H -f 3

"###,
        )
        .arg(
            Arg::new("runlist")
                .required(true)
                .index(1)
                .num_args(1)
                .help("Set the runlist file to use")
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .index(2)
                .num_args(1..)
                .help("Set the input files to use")
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
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
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
            Arg::new("op")
                .long("op")
                .num_args(1)
                .default_value("overlap")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("operations: overlap, non-overlap or superset"),
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

    let op = args.get_one::<String>("op").unwrap().as_str();

    let is_sharp = args.get_flag("sharp");
    let is_header = args.get_flag("header");

    let idx_range = if args.contains_id("field") {
        *args.get_one::<usize>("field").unwrap()
    } else {
        0
    };

    //----------------------------
    // Loading
    //----------------------------
    let json = read_json(args.get_one::<String>("runlist").unwrap());
    let set = json2set(&json);

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
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
