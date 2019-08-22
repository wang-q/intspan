use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("split")
        .about("Split a runlist yaml file")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("outdir")
                .short("o")
                .long("outdir")
                .takes_value(true)
                .default_value("stdout")
                .empty_values(false)
                .help("Output location. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let master: BTreeMap<String, Value> = read_runlist(args.value_of("infile").unwrap());

    let outdir = args.value_of("outdir").unwrap();
    if outdir != "stdout" {
        fs::create_dir_all(outdir);
    }

    //----------------------------
    // Operating
    //----------------------------
    for (key, value) in &master {
        if !value.is_mapping() {
            panic!("Not a valid multi-key runlist yaml file");
        }

        let string = serde_yaml::to_string(value).unwrap();

        //----------------------------
        // Output
        //----------------------------
        if outdir == "stdout" {
            write_lines("stdout", &vec![string.as_str()]);
        } else {
            let path = Path::new(outdir).join(key.to_owned() + ".yml");
            fs::write(path, string + "\n");
        }
    }
}
