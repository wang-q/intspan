use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("field")
        .about("Create/append ranges from fields")
        .after_help(
            r###"
Example:

    rgr field tests/Atha/chr.sizes --chr 1 --start 2 -a -s

    rgr field tests/spanr/NC_007942.gff -H --chr 1 --start 4 --end 5 --strand 7 --eq 3:tRNA --ne '7:+'

    rgr field tests/rgr/ctg.tsv --chr 2 --start 3 --end 4 -H -f 6,1

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
        )
        .arg(
            Arg::new("append")
                .long("append")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Append a field of range. The default is to write only the range"),
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
            Arg::new("fields")
                .long("fields")
                .short('f')
                .num_args(1)
                .help("Writes selected fields and the generated range field, in the order listed"),
        )
        .arg(
            Arg::new("chr")
                .long("chr")
                .num_args(1)
                .required(true)
                .value_parser(value_parser!(usize))
                .help("Field idx of chr"),
        )
        .arg(
            Arg::new("strand")
                .long("strand")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Optional field idx of strand"),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .num_args(1)
                .required(true)
                .value_parser(value_parser!(usize))
                .help("Field idx of start"),
        )
        .arg(
            Arg::new("end")
                .long("end")
                .value_parser(value_parser!(usize))
                .num_args(1)
                .help("Optional field idx of end"),
        )
        .arg(
            Arg::new("eq")
                .long("eq")
                .action(ArgAction::Append)
                .help("Filter lines by field:STR, FIELD == STR"),
        )
        .arg(
            Arg::new("ne")
                .long("ne")
                .action(ArgAction::Append)
                .help("Filter lines by field:STR, FIELD != STR"),
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
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());

    let mut is_append = args.get_flag("append");
    let is_header = args.get_flag("header");
    let is_sharp = args.get_flag("sharp");

    let idx_chr = *args.get_one::<usize>("chr").unwrap();
    let idx_strand = if args.contains_id("strand") {
        *args.get_one::<usize>("strand").unwrap()
    } else {
        0
    };
    let idx_start = *args.get_one::<usize>("start").unwrap();
    let idx_end = if args.contains_id("end") {
        *args.get_one::<usize>("end").unwrap()
    } else {
        0
    };

    let fields: Vec<usize> = if args.contains_id("fields") {
        is_append = true;

        let mut ints: Vec<i32> = vec![];
        let parts: Vec<&str> = args
            .get_one::<String>("fields")
            .unwrap()
            .split(',')
            .collect();
        for p in parts {
            let intspan = IntSpan::from(p);
            intspan.elements().iter().for_each(|e| ints.push(*e));
        }

        ints.iter().map(|e| *e as usize).collect()
    } else {
        vec![]
    };

    let mut eq_of: BTreeMap<usize, String> = BTreeMap::new();
    if args.contains_id("eq") {
        for s in args.get_many::<String>("eq").unwrap() {
            let parts: Vec<&str> = s.splitn(2, ':').collect();

            if parts.len() != 2 {
                eprintln!("Need a valid value for --eq {}", s);
                std::process::exit(1)
            }

            let idx = parts.get(0).unwrap().parse::<usize>().unwrap();

            eq_of.insert(idx, parts.get(1).unwrap().to_string());
        }
    }
    let mut ne_of: BTreeMap<usize, String> = BTreeMap::new();
    if args.contains_id("ne") {
        for s in args.get_many::<String>("ne").unwrap() {
            let parts: Vec<&str> = s.splitn(2, ':').collect();

            if parts.len() != 2 {
                eprintln!("Need a valid value for --ne {}", s);
                std::process::exit(1)
            }

            let idx = parts.get(0).unwrap().parse::<usize>().unwrap();

            ne_of.insert(idx, parts.get(1).unwrap().to_string());
        }
    }

    //----------------------------
    // Loading
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for (i, line) in reader.lines().filter_map(|r| r.ok()).enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();

            // the header line
            if is_header && i == 0 {
                if is_append {
                    if fields.is_empty() {
                        writer.write_fmt(format_args!("{}\t{}\n", line, "range"))?;
                    } else {
                        let selected: Vec<String> = fields
                            .iter()
                            .map(|e| parts.get(*e - 1).unwrap().to_string())
                            .collect();

                        writer.write_fmt(format_args!("{}\t{}\n", selected.join("\t"), "range"))?;
                    }
                } else {
                    writer.write_fmt(format_args!("{}\n", "range"))?;
                }
                continue 'LINE;
            }

            if line.starts_with('#') {
                if is_sharp {
                    writer.write_fmt(format_args!("{}\n", line))?;
                }
                continue 'LINE;
            }

            // --eq and --ne
            if !eq_of.is_empty() {
                for (k, v) in &eq_of {
                    let val = parts.get(k - 1).unwrap();
                    if val.to_string() != *v {
                        continue 'LINE;
                    }
                }
            }
            if !ne_of.is_empty() {
                for (k, v) in &ne_of {
                    let val = parts.get(k - 1).unwrap();
                    if val.to_string() == *v {
                        continue 'LINE;
                    }
                }
            }

            // build ranges
            let chr = parts.get(idx_chr - 1).unwrap();
            let strand = if idx_strand == 0 {
                ""
            } else {
                parts.get(idx_strand - 1).unwrap()
            };
            let start = parts.get(idx_start - 1).unwrap().parse::<i32>().unwrap();
            let end = if idx_end == 0 {
                start
            } else {
                parts.get(idx_end - 1).unwrap().parse::<i32>().unwrap()
            };

            let rg = Range {
                name: "".to_string(),
                chr: chr.to_string(),
                strand: strand.to_string(),
                start,
                end,
            };

            //----------------------------
            // Output
            //----------------------------
            let new_line: String;
            if is_append {
                if fields.is_empty() {
                    new_line = format!("{}\t{}", parts.join("\t"), rg);
                } else {
                    let selected: Vec<String> = fields
                        .iter()
                        .map(|e| parts.get(*e - 1).unwrap().to_string())
                        .collect();

                    new_line = format!("{}\t{}", selected.join("\t"), rg);
                }
            } else {
                new_line = rg.to_string();
            }

            writer.write_fmt(format_args!("{}\n", new_line))?;
        }
    }

    Ok(())
}
