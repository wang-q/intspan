use clap::*;
use intspan::*;
use std::ffi::OsStr;
use std::io::BufRead;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("prop")
        .about("Proportion of the ranges intersecting a runlist file")
        .after_help(
            r###"
* Lines without a valid range will not be output
* Append fields
    * `prop`
    * `length`: length of the range
    * `size`: size of the intersection

Example:

    rgr prop tests/rgr/intergenic.yml tests/rgr/S288c.rg

    rgr prop tests/rgr/intergenic.yml tests/rgr/ctg.range.tsv -H -f 3 --prefix --full

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
            Arg::new("full")
                .long("full")
                .takes_value(false)
                .help("Also append `length` and `size` fields"),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .takes_value(false)
                .help("Prefix the basename of the runlist file if `--header` is set"),
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

    let is_full = args.is_present("full");
    let is_prefix = args.is_present("prefix");

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
                if is_prefix {
                    let prefix = Path::new(args.value_of("runlist").unwrap())
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .unwrap()
                        .split('.')
                        .next()
                        .unwrap()
                        .to_string();
                    if is_full {
                        writer.write_fmt(format_args!(
                            "{}\t{}{}\t{}{}\t{}{}\n",
                            line, prefix, "Prop", prefix, "Length", prefix, "Size"
                        ))?;
                    } else {
                        writer.write_fmt(format_args!("{}\t{}{}\n", line, prefix, "Prop"))?;
                    }
                } else {
                    if is_full {
                        writer.write_fmt(format_args!(
                            "{}\t{}\t{}\t{}\n",
                            line, "prop", "length", "size"
                        ))?;
                    } else {
                        writer.write_fmt(format_args!("{}\t{}\n", line, "prop"))?;
                    }
                }

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
            if set.contains_key(chr) {
                let intxn = set.get(chr).unwrap().intersect(&intspan);
                let prop = intxn.cardinality() as f32 / intspan.cardinality() as f32;

                if is_full {
                    writer.write_fmt(format_args!(
                        "{}\t{:.4}\t{}\t{}\n",
                        line,
                        prop,
                        intspan.cardinality(),
                        intxn.cardinality()
                    ))?;
                } else {
                    writer.write_fmt(format_args!("{}\t{:.4}\n", line, prop))?;
                }
            } else {
                if is_full {
                    writer.write_fmt(format_args!(
                        "{}\t{:.4}\t{}\t{}\n",
                        line,
                        0.0,
                        intspan.cardinality(),
                        0
                    ))?;
                } else {
                    writer.write_fmt(format_args!("{}\t{:.4}\n", line, 0.0))?;
                }
            }
        }
    }

    Ok(())
}
