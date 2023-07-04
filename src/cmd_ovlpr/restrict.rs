use clap::*;
use intspan::*;
use std::collections::HashSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("restrict")
        .about("Restrict overlaps to known pairs")
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("restrict")
                .required(true)
                .num_args(1)
                .index(2)
                .help("Two-column tsv file"),
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
    // Loading
    //----------------------------
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());
    let reader = reader(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Load restricts
    //----------------------------
    let mut restricts: HashSet<String> = HashSet::new();
    for line in read_lines(args.get_one::<String>("restrict").unwrap()) {
        let mut parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            parts.sort_unstable();
            restricts.insert(parts.join("\t"));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for line in reader.lines().filter_map(|r| r.ok()) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 13 {
            continue;
        }

        // f_id and g_id
        let mut pair = vec![fields[0].to_string(), fields[1].to_string()];
        pair.sort();

        if restricts.contains(pair.join("\t").as_str()) {
            writer.write_all((fields.join("\t") + "\n").as_ref())?;
        }
    }

    Ok(())
}
