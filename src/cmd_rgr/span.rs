use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("span")
        .about("Operate spans in a .tsv/.rg file")
        .after_help(
            r###"
This command is similar to `spanr span`, but the <infiles> represent chromosome ranges.

List of Operations

* General Ops (both, 5p, or 3p)
    * trim: Remove `N` integers from the ends of the range.
    * pad: Add `N` integers to the ends of the range.
* Directional Ops (5p or 3p)
    * shift: Shift a range by N toward the 5p or 3p end.
    * flank: Retrieve flank regions of size `N` from the range.
* Size-baed Ops
    * excise: Remove any ranges that are smaller than `N`.

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
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
                .help("Set the index of the range field. When not set, the first valid range will be used"),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .num_args(1)
                .action(ArgAction::Set)
                .value_parser([
                    builder::PossibleValue::new("trim"),
                    builder::PossibleValue::new("pad"),
                    builder::PossibleValue::new("shift"),
                    builder::PossibleValue::new("flank"),
                    builder::PossibleValue::new("excise"),
                ])
                .default_value("trim")
                .help("Select the operation to perform"),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .short('m')
                .num_args(1)
                .action(ArgAction::Set)
                .value_parser([
                    builder::PossibleValue::new("both"),
                    builder::PossibleValue::new("5p"),
                    builder::PossibleValue::new("3p"),
                ])
                .default_value("both")
                .help("Specify the operation mode"),
        )
        .arg(
            Arg::new("number")
                .long("number")
                .short('n')
                .num_args(1)
                .value_parser(value_parser!(i32))
                .default_value("0")
                .help("Specify the number of integers to trim, pad, shift, or flank"),
        )
        .arg(
            Arg::new("append")
                .long("append")
                .short('a')
                .action(ArgAction::SetTrue)
                .help(
                    "Append a field for the new range (default: only write the new range)",
                ),
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

    let is_header = args.get_flag("header");
    let is_sharp = args.get_flag("sharp");

    let idx_range = if args.contains_id("field") {
        *args.get_one::<usize>("field").unwrap()
    } else {
        0
    };

    let opt_op = args.get_one::<String>("op").unwrap().as_str();
    let opt_mode = args.get_one::<String>("mode").unwrap().as_str();
    let opt_number = *args.get_one::<i32>("number").unwrap();

    let is_append = args.get_flag("append");

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            // the header line
            if is_header && i == 0 {
                if is_append {
                    writer.write_fmt(format_args!("{}\t{}\n", line, "rg"))?;
                } else {
                    writer.write_fmt(format_args!("{}\n", "rg"))?;
                }
                continue 'LINE;
            }

            if line.starts_with('#') {
                if is_sharp {
                    writer.write_fmt(format_args!("{}\n", line))?;
                }
                continue 'LINE;
            }

            let mut rg = intspan::Range::new();
            {
                let parts: Vec<&str> = line.split('\t').collect();
                if idx_range == 0 {
                    for part in &parts {
                        let r = intspan::Range::from_str(part);
                        if r.is_valid() {
                            rg = r;
                            break;
                        }
                    }
                } else {
                    rg = intspan::Range::from_str(parts.get(idx_range - 1).unwrap());
                }
            }

            if !rg.is_valid() {
                continue 'LINE;
            }

            let new = match opt_op {
                "trim" => {
                    if opt_mode == "5p" {
                        rg.trim_5p(opt_number)
                    } else if opt_mode == "3p" {
                        rg.trim_3p(opt_number)
                    } else {
                        rg.trim(opt_number)
                    }
                }
                "pad" => {
                    if opt_mode == "5p" {
                        rg.trim_5p(-opt_number)
                    } else if opt_mode == "3p" {
                        rg.trim_3p(-opt_number)
                    } else {
                        rg.trim(-opt_number)
                    }
                }
                "shift" => {
                    if opt_mode == "5p" {
                        rg.shift_5p(opt_number)
                    } else if opt_mode == "3p" {
                        rg.shift_3p(opt_number)
                    } else {
                        unreachable!("Invalid mode")
                    }
                }
                "flank" => {
                    if opt_mode == "5p" {
                        rg.flank_5p(opt_number)
                    } else if opt_mode == "3p" {
                        rg.flank_3p(opt_number)
                    } else {
                        unreachable!("Invalid mode")
                    }
                }
                "excise" => {
                    let size = rg.intspan().size();
                    if size >= opt_number {
                        rg.clone()
                    } else {
                        intspan::Range::new()
                    }
                }
                _ => unreachable!("Invalid Op"),
            };

            //----------------------------
            // Output
            //----------------------------
            let new_line: String = if is_append {
                format!("{}\t{}", line, new)
            } else {
                new.to_string()
            };

            writer.write_fmt(format_args!("{}\n", new_line))?;
        }
    }

    Ok(())
}