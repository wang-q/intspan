use clap::*;
use std::ffi::OsStr;
use std::io::BufRead;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("prop")
        .about("Proportion of the ranges intersecting a runlist file")
        .after_help(
            r###"
* Lines without a valid range will not be output
* Appended fields
    * `prop`
    * `length`: length of the range (if `--full` is set)
    * `size`: size of the intersection (if `--full` is set)

Example:

    rgr prop tests/rgr/intergenic.json tests/rgr/S288c.rg

    rgr prop tests/rgr/intergenic.json tests/rgr/ctg.range.tsv -H -f 3 --prefix --full

"###,
        )
        .arg(
            Arg::new("runlist")
                .required(true)
                .index(1)
                .num_args(1)
                .help("Runlist file to calculate intersections against"),
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .index(2)
                .num_args(1..)
                .help("Input files to process. Multiple files can be specified"),
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
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Index of the range field. If not set, the first valid range will be used"),
        )
        .arg(
            Arg::new("full")
                .long("full")
                .action(ArgAction::SetTrue)
                .help("Also append `length` and `size` fields"),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .action(ArgAction::SetTrue)
                .help("Prefix the basename of the runlist file if `--header` is set"),
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
    // Args
    //----------------------------
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());

    let is_sharp = args.get_flag("sharp");
    let is_header = args.get_flag("header");

    let opt_idx_range = args.get_one::<usize>("field").copied().unwrap_or(0);

    let is_full = args.get_flag("full");
    let is_prefix = args.get_flag("prefix");

    //----------------------------
    // Loading
    //----------------------------
    let json = intspan::read_json(args.get_one::<String>("runlist").unwrap());
    let set = intspan::json2set(&json);

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Handle the header line
            if is_header && i == 0 {
                if is_prefix {
                    let prefix = Path::new(args.get_one::<String>("runlist").unwrap())
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
                } else if is_full {
                    writer.write_fmt(format_args!(
                        "{}\t{}\t{}\t{}\n",
                        line, "prop", "length", "size"
                    ))?;
                } else {
                    writer.write_fmt(format_args!("{}\t{}\n", line, "prop"))?;
                }

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

            // Calculate intersection
            let chr = rg.chr();
            let mut intspan = intspan::IntSpan::new();
            intspan.add_pair(*rg.start(), *rg.end());

            let (prop, length, size) = if set.contains_key(chr) {
                let intxn = set.get(chr).unwrap().intersect(&intspan);
                let prop = intxn.cardinality() as f32 / intspan.cardinality() as f32;
                (prop, intspan.cardinality(), intxn.cardinality())
            } else {
                (0.0, intspan.cardinality(), 0)
            };

            //----------------------------
            // Output
            //----------------------------
            if is_full {
                writer.write_fmt(format_args!(
                    "{}\t{:.4}\t{}\t{}\n",
                    line, prop, length, size
                ))?;
            } else {
                writer.write_fmt(format_args!("{}\t{:.4}\n", line, prop))?;
            }
        }
    }

    Ok(())
}
