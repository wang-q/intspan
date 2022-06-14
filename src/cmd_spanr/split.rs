use clap::*;
use intspan::*;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("split")
        .about("Split a runlist yaml file")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("suffix")
                .long("suffix")
                .short('s')
                .takes_value(true)
                .default_value(".yml")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Extensions of output files"),
        )
        .arg(
            Arg::new("outdir")
                .short('o')
                .long("outdir")
                .takes_value(true)
                .default_value("stdout")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Output location. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //----------------------------
    // Loading
    //----------------------------
    let yaml: BTreeMap<String, Value> = read_yaml(args.get_one::<String>("infile").unwrap());

    let outdir = args.get_one::<String>("outdir").unwrap();
    if outdir != "stdout" {
        fs::create_dir_all(outdir)?;
    }

    let suffix = args.get_one::<String>("suffix").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for (key, value) in &yaml {
        if !value.is_mapping() {
            panic!("Not a valid multi-key runlist yaml file");
        }

        let string = serde_yaml::to_string(value).unwrap();

        //----------------------------
        // Output
        //----------------------------
        if outdir == "stdout" {
            write_lines("stdout", &vec![string.as_str()])?;
        } else {
            let path = Path::new(outdir).join(key.to_owned() + suffix);
            fs::write(path, string + "\n")?;
        }
    }

    Ok(())
}
