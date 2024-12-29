use clap::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("filter")
        .about("Filter lines in .tsv files via tests against individual fields")
        .after_help(
            r###"
Examples:

    rgr filter tests/spanr/NC_007942.gff -H --str-eq 3:tRNA --str-ne '7:+'
    rgr filter tests/spanr/NC_007942.gff -H -c --str-eq 3:trna --str-ne '7:+'

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
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
        )
        .arg(
            Arg::new("or")
                .long("or")
                .action(ArgAction::SetTrue)
                .help("Evaluate tests as an OR rather than an AND clause"),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Invert the filter"),
        )
        .arg(
            Arg::new("case")
                .long("case")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Case insensitive"),
        )
        .arg(
            Arg::new("str-eq")
                .long("str-eq")
                .action(ArgAction::Append)
                .help("Filter lines by field:STR, FIELD == STR"),
        )
        .arg(
            Arg::new("str-ne")
                .long("str-ne")
                .action(ArgAction::Append)
                .help("Filter lines by field:STR, FIELD != STR"),
        )
        .arg(
            Arg::new("eq")
                .long("eq")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD == NUM"),
        )
        .arg(
            Arg::new("ne")
                .long("ne")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD != NUM"),
        )
        .arg(
            Arg::new("lt")
                .long("lt")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD < NUM"),
        )
        .arg(
            Arg::new("le")
                .long("le")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD <= NUM"),
        )
        .arg(
            Arg::new("gt")
                .long("gt")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD > NUM"),
        )
        .arg(
            Arg::new("ge")
                .long("ge")
                .action(ArgAction::Append)
                .help("Filter lines by field:NUM, FIELD >= NUM"),
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
    let is_or = args.get_flag("or");
    let is_invert = args.get_flag("invert");
    let is_insensitive = args.get_flag("case");

    let str_eq_tpl = opt_fields_str(args, "str-eq", is_insensitive);
    let str_ne_tpl = opt_fields_str(args, "str-ne", is_insensitive);

    let num_eq_tpl = opt_fields_num(args, "eq");
    let num_ne_tpl = opt_fields_num(args, "ne");
    let num_lt_tpl = opt_fields_num(args, "lt");
    let num_le_tpl = opt_fields_num(args, "le");
    let num_gt_tpl = opt_fields_num(args, "gt");
    let num_ge_tpl = opt_fields_num(args, "ge");

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();

            // the header line
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

            // filters
            let mut pass_ary = vec![];

            // --str-eq and --str-ne
            if !str_eq_tpl.is_empty() {
                for (k, v) in &str_eq_tpl {
                    let val = parts.get(k - 1).unwrap();
                    let pass = if is_insensitive {
                        *val.to_ascii_uppercase() == *v
                    } else {
                        *val == *v
                    };
                    pass_ary.push(pass);
                }
            }
            if !str_ne_tpl.is_empty() {
                for (k, v) in &str_ne_tpl {
                    let val = parts.get(k - 1).unwrap();
                    let pass = if is_insensitive {
                        *val.to_ascii_uppercase() != *v
                    } else {
                        *val != *v
                    };
                    pass_ary.push(pass);
                }
            }

            // --eq, --ne, --gt and --ge
            if !num_eq_tpl.is_empty() {
                for (k, v) in &num_eq_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val == *v;
                    pass_ary.push(pass);
                }
            }
            if !num_ne_tpl.is_empty() {
                for (k, v) in &num_ne_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val != *v;
                    pass_ary.push(pass);
                }
            }
            if !num_lt_tpl.is_empty() {
                for (k, v) in &num_lt_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val < *v;
                    pass_ary.push(pass);
                }
            }
            if !num_le_tpl.is_empty() {
                for (k, v) in &num_le_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val <= *v;
                    pass_ary.push(pass);
                }
            }
            if !num_gt_tpl.is_empty() {
                for (k, v) in &num_gt_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val > *v;
                    pass_ary.push(pass);
                }
            }
            if !num_ge_tpl.is_empty() {
                for (k, v) in &num_ge_tpl {
                    let val = parts.get(k - 1).unwrap().parse::<f64>().unwrap();
                    let pass = val >= *v;
                    pass_ary.push(pass);
                }
            }

            // combine bools
            let mut flag_pass = if is_or {
                pass_ary.iter().any(|&b| b)
            } else {
                pass_ary.iter().all(|&b| b)
            };
            if is_invert {
                flag_pass = !flag_pass;
            }

            //----------------------------
            // Output
            //----------------------------
            if flag_pass {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
        }
    }

    Ok(())
}

fn opt_fields_str(args: &ArgMatches, id: &str, is_insensitive: bool) -> Vec<(usize, String)> {
    let mut str_cmp_tpl: Vec<(usize, String)> = Vec::new();
    if args.contains_id(id) {
        for s in args.get_many::<String>(id).unwrap() {
            let parts: Vec<&str> = s.splitn(2, ':').collect();

            if parts.len() != 2 {
                eprintln!("Need a valid value for --{} {}", id, s);
                std::process::exit(1)
            }

            let fields = intspan::ints_to_idx(parts.first().unwrap());

            for idx in &fields {
                if is_insensitive {
                    str_cmp_tpl
                        .push((*idx, parts.get(1).unwrap().to_string().to_ascii_uppercase()));
                } else {
                    str_cmp_tpl.push((*idx, parts.get(1).unwrap().to_string()));
                }
            }
        }
    }
    str_cmp_tpl
}

fn opt_fields_num(args: &ArgMatches, id: &str) -> Vec<(usize, f64)> {
    let mut num_cmp_tpl: Vec<(usize, f64)> = Vec::new();
    if args.contains_id(id) {
        for s in args.get_many::<String>(id).unwrap() {
            let parts: Vec<&str> = s.splitn(2, ':').collect();

            if parts.len() != 2 {
                eprintln!("Need a valid value for --{} {}", id, s);
                std::process::exit(1)
            }

            let fields = intspan::ints_to_idx(parts.first().unwrap());

            for idx in &fields {
                num_cmp_tpl.push((*idx, parts.get(1).unwrap().parse::<f64>().unwrap()));
            }
        }
    }
    num_cmp_tpl
}
