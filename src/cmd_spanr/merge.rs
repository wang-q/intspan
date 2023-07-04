use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("merge")
        .about("Merge runlist json files")
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("All parts of file_stem (aka basename), except the last one"),
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
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Loading
    //----------------------------
    let mut out_json: BTreeMap<String, Value> = BTreeMap::new();

    let is_all = args.contains_id("all");

    for infile in args.get_many::<String>("infiles").unwrap() {
        let json = read_json(infile);

        let key = if is_all {
            Path::new(infile)
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap()
                .to_string()
        } else {
            Path::new(infile)
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string()
        };
        out_json.insert(key, serde_json::to_value(json).unwrap());
    }

    //----------------------------
    // Output
    //----------------------------
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
