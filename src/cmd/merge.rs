use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("merge")
        .about("Merge runlist yaml files")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
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
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let mut out_yaml: BTreeMap<String, Value> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let yaml = read_yaml(infile);
        out_yaml.insert(
            Path::new(infile)
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap()
                .to_string(),
            serde_yaml::to_value(yaml).unwrap(),
        );
    }

    //----------------------------
    // Output
    //----------------------------
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
