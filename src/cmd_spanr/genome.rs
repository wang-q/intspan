use clap::*;
use intspan::*;
use serde_yaml::Value;
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
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //----------------------------
    // Loading
    //----------------------------
    let sizes = read_sizes(args.value_of("infile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    let mut yaml: BTreeMap<String, Value> = BTreeMap::new();
    for (key, value) in sizes {
        let mut intspan = IntSpan::new();
        intspan.add_pair(1, value);
        yaml.insert(key, intspan.to_string().into());
    }

    //----------------------------
    // Output
    //----------------------------
    write_yaml(args.value_of("outfile").unwrap(), &yaml)?;

    Ok(())
}
