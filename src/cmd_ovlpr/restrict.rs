use clap::*;
use intspan::*;
use std::collections::HashSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("restrict")
        .about("Restrict overlaps to known pairs")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("restrict")
                .help("Two-column tsv file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .empty_values(false)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
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
        pair.push(fields[0].clone());
        pair.push(fields[1].clone());
        pair.sort();

        if restricts.contains(pair.join("\t").as_str()) {
            writer.write_all((fields.join("\t") + "\n").as_ref())?;
        }
    }

    Ok(())
}
