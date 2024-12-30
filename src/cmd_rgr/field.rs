use clap::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("field")
        .about("Create/append ranges from fields")
        .after_help(
            r###"
Examples:

1. Create ranges from a chromosome size file:
    rgr field tests/Atha/chr.sizes --chr 1 --start 2 -a -s

2. Create ranges from a GFF file:
    rgr field tests/spanr/NC_007942.gff -H --chr 1 --start 4 --end 5 --strand 7

3. Create ranges from a .tsv file:
    rgr field tests/rgr/ctg.tsv --chr 2 --start 3 --end 4 -H

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Input files to process"),
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
            Arg::new("chr")
                .long("chr")
                .num_args(1)
                .required(true)
                .value_parser(value_parser!(usize))
                .help("Field index for chr"),
        )
        .arg(
            Arg::new("strand")
                .long("strand")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Optional field index for strand"),
        )
        .arg(
            Arg::new("start")
                .long("start")
                .num_args(1)
                .required(true)
                .value_parser(value_parser!(usize))
                .help("Field index for start"),
        )
        .arg(
            Arg::new("end")
                .long("end")
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Optional field index for end"),
        )
        .arg(
            Arg::new("append")
                .long("append")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Append a field for the range (default: only write the range)"),
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

    let opt_idx_chr = *args.get_one::<usize>("chr").unwrap();
    let opt_idx_strand = args.get_one::<usize>("strand").copied().unwrap_or(0);
    let opt_idx_start = *args.get_one::<usize>("start").unwrap();
    let opt_idx_end = args.get_one::<usize>("end").copied().unwrap_or(0);

    let is_append = args.get_flag("append");

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();

            // Handle the header line
            if is_header && i == 0 {
                if is_append {
                    writer.write_fmt(format_args!("{}\t{}\n", line, "range"))?;
                } else {
                    writer.write_fmt(format_args!("{}\n", "range"))?;
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

            // Build ranges
            let chr = parts.get(opt_idx_chr - 1).unwrap();
            let strand = if opt_idx_strand == 0 {
                ""
            } else {
                parts.get(opt_idx_strand - 1).unwrap()
            };
            let start = parts
                .get(opt_idx_start - 1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let end = if opt_idx_end == 0 {
                start
            } else {
                parts.get(opt_idx_end - 1).unwrap().parse::<i32>().unwrap()
            };

            let rg = intspan::Range {
                name: "".to_string(),
                chr: chr.to_string(),
                strand: strand.to_string(),
                start,
                end,
            };

            //----------------------------
            // Output
            //----------------------------
            let new_line: String = if is_append {
                format!("{}\t{}", parts.join("\t"), rg)
            } else {
                rg.to_string()
            };

            writer.write_fmt(format_args!("{}\n", new_line))?;
        }
    }

    Ok(())
}
