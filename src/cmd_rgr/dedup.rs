use clap::*;
use std::collections::HashSet;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("dedup")
        .about("Deduplicate lines in .tsv file(s)")
        .after_help(
            r###"
The file requires a single pass without sorting, with each line consuming 8 bytes (u64) of memory.
As a trade-off, this program cannot count the occurrences.

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input file(s) to use"),
        )
        .arg(
            Arg::new("fields")
                .long("fields")
                .short('f')
                .num_args(1)
                .help("Fields to use as the key"),
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

    let opt_fields: intspan::IntSpan = if args.contains_id("fields") {
        intspan::fields_to_ints(args.get_one::<String>("fields").unwrap())
    } else {
        intspan::IntSpan::new()
    };

    //----------------------------
    // Ops
    //----------------------------
    let mut subject_set: HashSet<u64> = HashSet::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);

        for line in reader.lines().map_while(Result::ok) {
            let subject = if opt_fields.is_empty() {
                // whole line
                xxhash_rust::xxh3::xxh3_64(&line.clone().into_bytes())
            } else {
                // Get elements at specified indices
                let fields: Vec<&str> = line.split('\t').collect();
                let subset: Vec<&str> = opt_fields
                    .elements()
                    .iter()
                    .filter_map(|&i| fields.get(i as usize - 1))
                    .copied()
                    .collect();
                let concat = subset.join("\t");
                xxhash_rust::xxh3::xxh3_64(&concat.into_bytes())
            };

            if !subject_set.contains(&subject) {
                writer.write_fmt(format_args!("{}\n", line))?;
                subject_set.insert(subject);
            }
        }
    }

    Ok(())
}
