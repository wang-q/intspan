use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("split")
        .about("Split a runlist json file")
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
                .num_args(1)
                .default_value(".json")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Extensions of output files"),
        )
        .arg(
            Arg::new("outdir")
                .short('o')
                .long("outdir")
                .num_args(1)
                .default_value("stdout")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Output location. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Loading
    //----------------------------
    let json: BTreeMap<String, Value> = read_json(args.get_one::<String>("infile").unwrap());

    let outdir = args.get_one::<String>("outdir").unwrap();
    if outdir != "stdout" {
        fs::create_dir_all(outdir)?;
    }

    let suffix = args.get_one::<String>("suffix").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for (key, value) in &json {
        if !value.is_object() {
            panic!("Not a valid multi-key runlist json file");
        }

        let string = serde_json::to_string(value).unwrap();

        //----------------------------
        // Output
        //----------------------------
        if outdir == "stdout" {
            write_lines("stdout", &vec![string])?;
        } else {
            let path = Path::new(outdir).join(key.to_owned() + suffix);
            fs::write(path, string + "\n")?;
        }
    }

    Ok(())
}
