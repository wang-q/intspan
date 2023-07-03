use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("genome")
        .about("Convert chr.size to runlists")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
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
    let sizes = read_sizes(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    let mut json: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in sizes {
        let mut intspan = IntSpan::new();
        intspan.add_pair(1, value);
        json.insert(key, intspan.to_string().into());
    }

    //----------------------------
    // Output
    //----------------------------
    write_json(args.get_one::<String>("outfile").unwrap(), &json)?;

    Ok(())
}
