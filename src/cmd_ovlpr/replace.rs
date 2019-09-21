use clap::*;
use intspan::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("replace")
        .about("Replace IDs in .ovlp.tsv")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("replace")
                .help("Two-column tsv file, From-To")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("reverse")
                .long("reverse")
                .short("r")
                .help("To-From instead of From-To in .replace.tsv"),
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
    // Load replaces
    //----------------------------
    let mut replaces: HashMap<String, String> = HashMap::new();
    for line in read_lines(args.value_of("replace").unwrap()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            if args.is_present("reverse") {
                replaces.insert(parts[1].to_string(), parts[0].to_string());
            } else {
                replaces.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for line in reader.lines().filter_map(|r| r.ok()) {
        let mut fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 13 {
            continue;
        }

        // f_id
        if replaces.contains_key(fields[0]) {
            fields[0] = replaces.get(fields[0]).unwrap();
        }
        // g_id
        if replaces.contains_key(fields[1]) {
            fields[1] = replaces.get(fields[1]).unwrap();
        }

        writer.write_all((fields.join("\t") + "\n").as_ref())?;
    }

    Ok(())
}
