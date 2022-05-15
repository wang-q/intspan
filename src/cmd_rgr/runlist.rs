use clap::*;
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("runlist")
        .about("Filter a range file by comparison with a runlist file")
        .arg(
            Arg::new("runlist")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("ranges")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .takes_value(true)
                .default_value("overlap")
                .forbid_empty_values(true)
                .help("operations: overlap, non-overlap or superset"),
        )
        .arg(
            Arg::new("sharp")
                .long("sharp")
                .short('s')
                .takes_value(false)
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
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
    // Options
    //----------------------------
    let op = args.value_of("op").unwrap();
    let is_sharp = args.is_present("sharp");

    //----------------------------
    // Loading
    //----------------------------
    let yaml = read_yaml(args.value_of("runlist").unwrap());
    let set = yaml2set(&yaml);

    let reader = reader(args.value_of("ranges").unwrap());
    let mut writer = writer(args.value_of("outfile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    'LINE: for line in reader.lines().filter_map(|r| r.ok()) {
        if line.starts_with('#') {
            if is_sharp {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
            continue 'LINE;
        }

        let range = Range::from_str(&line);
        if !range.is_valid() {
            continue 'LINE;
        }
        let chr = range.chr();
        let mut intspan = IntSpan::new();
        intspan.add_pair(*range.start(), *range.end());

        //----------------------------
        // Output
        //----------------------------
        match op {
            "overlap" => {
                if set.contains_key(chr) && !set.get(chr).unwrap().intersect(&intspan).is_empty() {
                    writer.write_all((line + "\n").as_ref())?;
                }
            }
            "non-overlap" => {
                if set.contains_key(chr) {
                    if set.get(chr).unwrap().intersect(&intspan).is_empty() {
                        writer.write_all((line + "\n").as_ref())?;
                    }
                } else {
                    writer.write_all((line + "\n").as_ref())?;
                }
            }
            "superset" => {
                if set.contains_key(chr) && set.get(chr).unwrap().superset(&intspan) {
                    writer.write_all((line + "\n").as_ref())?;
                }
            }
            _ => panic!("Invalid Op"),
        };
    }

    Ok(())
}
