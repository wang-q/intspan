use clap::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("sort")
        .about("Sort .rg and .tsv files by a range field")
        .after_help(
            r###"
* If no part of the line is a valid range, the line will be written to to the end of the output

* Using `--group` can improve performance on large datasets by grouping rows before sorting.
    * The group_key can be chr_id, ctg_id, etc.

Example:

    # Sort a .rg file
    rgr sort tests/rgr/S288c.rg

    # Sort a .tsv file by the first valid range
    rgr sort tests/rgr/ctg.range.tsv

    # Sort a .tsv file by a specific range field and treat the first line as a header
    rgr sort tests/rgr/ctg.range.tsv -H -f 3

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
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
            Arg::new("field")
                .long("field")
                .short('f')
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Index of the range field. If not set, the first valid range will be used"),
        )
        .arg(
            Arg::new("group")
                .long("group")
                .short('g')
                .num_args(1)
                .value_parser(value_parser!(usize))
                .help("Group the rows by this field and then sort within each group"),
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

    let is_header = args.get_flag("header");

    let opt_idx_range = args.get_one::<usize>("field").copied().unwrap_or(0);
    let opt_idx_group = args.get_one::<usize>("group").copied().unwrap_or(0);

    //----------------------------
    // Loading
    //----------------------------
    let mut line_to_rg: BTreeMap<String, intspan::Range> = BTreeMap::new();
    let mut invalids: Vec<String> = vec![];

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Handle the header line
            if is_header && i == 0 {
                writer.write_fmt(format_args!("{}\n", line))?;
                continue 'LINE;
            }

            // Extract the range
            let parts: Vec<&str> = line.split('\t').collect();

            let range = if opt_idx_range == 0 {
                parts.iter().find_map(|part| {
                    let rg = intspan::Range::from_str(part);
                    if rg.is_valid() {
                        Some(rg)
                    } else {
                        None
                    }
                })
            } else {
                let part = parts.get(opt_idx_range - 1).unwrap();
                let rg = intspan::Range::from_str(part);
                if rg.is_valid() {
                    Some(rg)
                } else {
                    None
                }
            };

            // Store the line and its range
            if let Some(range) = range {
                line_to_rg.insert(line.clone(), range);
            } else {
                invalids.push(line.clone()); // No valid range found
            }
        }
    }

    //----------------------------
    // Sorting
    //----------------------------
    let mut sorted: Vec<String> = vec![];

    if opt_idx_group == 0 {
        // Sort all lines together
        sorted = line_to_rg.keys().map(|e| e.to_string()).collect();

        sorted.sort_by_cached_key(|k| {
            let range = line_to_rg.get(k).unwrap();
            (range.chr().clone(), range.start(), range.strand().clone())
        });
    } else {
        // Group lines by the specified field, then sort within each group
        let mut lines_of: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for line in line_to_rg.keys() {
            let parts: Vec<&str> = line.split('\t').collect();

            let group_key = parts.get(opt_idx_group - 1).unwrap();
            lines_of
                .entry(group_key.to_string())
                .or_default()
                .push(line.clone());
        }

        for group_key in lines_of.keys().sorted() {
            let mut lines = lines_of.get(group_key).unwrap().clone();

            lines.sort_by_cached_key(|k| {
                let range = line_to_rg.get(k).unwrap();
                (range.chr().clone(), range.start(), range.strand().clone())
            });
            sorted.extend(lines);
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for line in &sorted {
        writer.write_fmt(format_args!("{}\n", line))?;
    }
    for line in &invalids {
        writer.write_fmt(format_args!("{}\n", line))?;
    }

    Ok(())
}
