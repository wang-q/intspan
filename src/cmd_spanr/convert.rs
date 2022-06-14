use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("convert")
        .about("Convert runlist file to ranges file")
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let yaml = read_yaml(infile);
        let set_of = yaml2set_m(&yaml);

        for set in set_of.values() {
            for chr in set.keys() {
                let intspan = set.get(chr).unwrap();
                let span_size = intspan.span_size();
                let ranges = intspan.ranges();
                for i in 0..span_size {
                    let lower = *ranges.get(i * 2).unwrap();
                    let upper = *ranges.get(i * 2 + 1).unwrap();

                    //----------------------------
                    // Output
                    //----------------------------
                    if lower == upper {
                        writer.write_all(format!("{}:{}\n", chr, lower).as_ref())?;
                    } else {
                        writer.write_all(format!("{}:{}-{}\n", chr, lower, upper).as_ref())?;
                    }
                }
            }
        }
    }

    Ok(())
}
