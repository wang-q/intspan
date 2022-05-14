use clap::*;
use intspan::*;
use std::collections::HashSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("restrict")
        .about("Restrict overlaps to known pairs")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("restrict")
                .help("Two-column tsv file")
                .required(true)
                .index(2),
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
    //----------------------------
    // Loading
    //----------------------------
    let mut writer = writer(args.value_of("outfile").unwrap());
    let reader = reader(args.value_of("infile").unwrap());

    //----------------------------
    // Load restricts
    //----------------------------
    let mut restricts: HashSet<String> = HashSet::new();
    for line in read_lines(args.value_of("restrict").unwrap()) {
        let mut parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            parts.sort();
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
        let mut pair = vec![];
        pair.push(fields[0].to_string());
        pair.push(fields[1].to_string());
        pair.sort();

        if restricts.contains(pair.join("\t").as_str()) {
            writer.write_all((fields.join("\t") + "\n").as_ref())?;
        }
    }

    Ok(())
}
