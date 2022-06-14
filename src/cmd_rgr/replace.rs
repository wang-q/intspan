use clap::*;
use intspan::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("replace")
        .about("Replace fields in .tsv file")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("replace")
                .help("A two-column tsv file, From--To")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('r')
                .help("To--From instead in .replace.tsv"),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //----------------------------
    // Loading
    //----------------------------
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());
    let reader = reader(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Load replaces
    //----------------------------
    let mut replaces: HashMap<String, String> = HashMap::new();
    for line in read_lines(args.get_one::<String>("replace").unwrap()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            if args.contains_id("reverse") {
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

        for i in 0..fields.len() {
            if replaces.contains_key(fields[i]) {
                fields[i] = replaces.get(fields[i]).unwrap();
            }
        }

        writer.write_all((fields.join("\t") + "\n").as_ref())?;
    }

    Ok(())
}
