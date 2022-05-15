use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("field")
        .about("Create/append ranges from fields")
        .after_help(
            r###"
Field numbers start with 1.

Example:

    rgr field tests/Atha/chr.sizes --chr 1 --start 2 -a -s

    rgr field tests/spanr/NC_007942.gff -H --chr 1 --start 4 --end 5 --strand 7 --eq 3:tRNA --ne '7:+'

    rgr field tests/rgr/ctg.tsv --chr 2 --start 3 --end 4 -H -f 6,1

"###,
        )
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("append")
                .long("append")
                .short('a')
                .takes_value(false)
                .help("Append a field of range. The default is to write only the range"),
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
            Arg::new("fields")
                .long("fields")
                .short('f')
                .takes_value(true)
                .help("Writes selected fields and the generated range field, in the order listed"),
        )
        .arg(
            Arg::new("chr")
                .long("chr")
                .takes_value(true)
                .required(true)
                .help("Field idx of chr"),
        )
        .arg(
            Arg::new("strand")
                .long("strand")
                .takes_value(true)
                .help("Optional field idx of strand"),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .takes_value(true)
                .required(true)
                .help("Field idx of start"),
        )
        .arg(
            Arg::new("end")
                .long("end")
                .takes_value(true)
                .help("Optional field idx of end"),
        )
        .arg(
            Arg::new("eq")
                .long("eq")
                .takes_value(true)
                .multiple_occurrences(true)
                .help("Filter lines by field:STR, FIELD == STR"),
        )
        .arg(
            Arg::new("ne")
                .long("ne")
                .takes_value(true)
                .multiple_occurrences(true)
                .help("Filter lines by field:STR, FIELD != STR"),
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
    let mut writer = intspan::writer(args.value_of("outfile").unwrap());

    //----------------------------
    // Options
    //----------------------------
    let mut is_append = args.is_present("append");
    let is_header = args.is_present("header");
    let is_sharp = args.is_present("sharp");

    let idx_chr: usize = args.value_of_t("chr").unwrap_or_else(|e| {
        eprintln!("Need an integer for --chr\n{}", e);
        std::process::exit(1)
    });
    let idx_strand: usize = if args.is_present("strand") {
        args.value_of_t("strand").unwrap_or_else(|e| {
            eprintln!("Need an integer for --strand\n{}", e);
            std::process::exit(1)
        })
    } else {
        0
    };
    let idx_start: usize = args.value_of_t("start").unwrap_or_else(|e| {
        eprintln!("Need an integer for --start\n{}", e);
        std::process::exit(1)
    });
    let idx_end: usize = if args.is_present("end") {
        args.value_of_t("end").unwrap_or_else(|e| {
            eprintln!("Need an integer for --end\n{}", e);
            std::process::exit(1)
        })
    } else {
        0
    };

    let fields: Vec<usize> = if args.is_present("fields") {
        is_append = true;

        let mut ints: Vec<i32> = vec![];
        let parts: Vec<&str> = args.value_of("fields").unwrap().split(',').collect();
        for p in parts {
            let intspan = IntSpan::from(p);
            intspan.elements().iter().for_each(|e| ints.push(*e));
        }

        ints.iter().map(|e| *e as usize).collect()
    } else {
        vec![]
    };

    let mut eq_of: BTreeMap<usize, String> = BTreeMap::new();
    if args.is_present("eq") {
        for s in args.values_of("eq").unwrap() {
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
    if args.is_present("ne") {
        for s in args.values_of("ne").unwrap() {
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
    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        'line: for (i, line) in reader.lines().filter_map(|r| r.ok()).enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();

            // the header line
            if is_header {
                if i == 0 {
                    if is_append {
                        if fields.is_empty() {
                            writer.write_fmt(format_args!("{}\t{}\n", line, "range"))?;
                        } else {
                            let selected: Vec<String> = fields
                                .iter()
                                .map(|e| parts.get(*e - 1).unwrap().to_string())
                                .collect();

                            writer.write_fmt(format_args!(
                                "{}\t{}\n",
                                selected.join("\t"),
                                "range"
                            ))?;
                        }
                    } else {
                        writer.write_fmt(format_args!("{}\n", "range"))?;
                    }
                    continue 'line;
                }
            }

            if line.starts_with('#') {
                if is_sharp {
                    writer.write_fmt(format_args!("{}\n", line))?;
                }
                continue 'line;
            }

            // --eq and --ne
            if !eq_of.is_empty() {
                for (k, v) in &eq_of {
                    let val = parts.get(k - 1).unwrap();
                    if val.to_string() != *v {
                        continue 'line;
                    }
                }
            }
            if !ne_of.is_empty() {
                for (k, v) in &ne_of {
                    let val = parts.get(k - 1).unwrap();
                    if val.to_string() == *v {
                        continue 'line;
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
