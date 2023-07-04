use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("merge")
        .about("Merge runlist json files")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .action(ArgAction::SetTrue)
                .help("All parts of file_stem (aka basename), except the last one"),
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
    let mut out_json: BTreeMap<String, Value> = BTreeMap::new();

    let is_all = args.get_flag("all");

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
