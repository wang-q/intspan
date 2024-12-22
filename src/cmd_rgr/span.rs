use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("span")
        .about("Operate spans in a range file")
        .after_help(
            r###"
* Like command `spanr span`, but <infiles> are chromosome ranges

List of Operations

* both, 5p, or 3p
    * trim: Remove `N` integers from the ends of the range.
    * pad: Add `N` integers to the ends of the range.
* 5p or 3p
    * shift: Shift a range by N toward 5p or 3p
    * flank: Retrieve flank regions of size `N` from the range.
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
                .help("Operations"),
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
                .help("Operation mode"),
        )
        .arg(
            Arg::new("number")
                .long("number")
                .short('n')
                .num_args(1)
                .value_parser(value_parser!(i32))
                .default_value("0"),
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
    let opt_mode = args.get_one::<String>("mode").unwrap().as_str();
    let opt_number = *args.get_one::<i32>("number").unwrap();

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        for line in reader.lines().map_while(Result::ok) {
            let rg = intspan::Range::from_str(&line);
            if !rg.is_valid() {
                continue;
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

            writer.write_fmt(format_args!("{}\n", new))?;
            if new.is_valid() {}
        }
    }

    Ok(())
}
