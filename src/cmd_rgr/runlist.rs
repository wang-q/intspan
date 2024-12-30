use clap::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("runlist")
        .about("Filter .rg and .tsv files by comparing with a runlist file")
        .after_help(
            r###"
* Lines without a valid range will not be output

Example:

    # Filter lines that overlap with the runlist
    rgr runlist tests/rgr/intergenic.json tests/rgr/S288c.rg --op overlap

    # # Filter lines that overlap with the runlist in a TSV file with headers
    rgr runlist tests/rgr/intergenic.json tests/rgr/ctg.range.tsv --op overlap -H -f 3

"###,
        )
        .arg(
            Arg::new("runlist")
                .required(true)
                .index(1)
                .num_args(1)
                .help("Set the runlist file to use"),
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
                .help("Preserve lines starting with a `#` without changes. The default is to ignore them"),
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
            Arg::new("op")
                .long("op")
                .num_args(1)
                .action(ArgAction::Set)
                .value_parser([
                    builder::PossibleValue::new("overlap"),
                    builder::PossibleValue::new("non-overlap"),
                    builder::PossibleValue::new("superset"),
                ])
                .default_value("overlap")
                .help("Filter operation: overlap, non-overlap or superset"),
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

    let opt_op = args.get_one::<String>("op").unwrap().as_str();

    let is_sharp = args.get_flag("sharp");
    let is_header = args.get_flag("header");

    let opt_idx_range = args.get_one::<usize>("field").copied().unwrap_or(0);

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
                writer.write_fmt(format_args!("{}\n", line))?;
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

            // Prepare the range for comparison
            let chr = rg.chr();
            let mut intspan = intspan::IntSpan::new();
            intspan.add_pair(*rg.start(), *rg.end());

            //----------------------------
            // Output
            //----------------------------
            match opt_op {
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
                _ => unreachable!("Invalid operation: {}", opt_op),
            };
        }
    }

    Ok(())
}
