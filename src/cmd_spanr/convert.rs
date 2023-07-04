use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("convert")
        .about("Convert runlist file to ranges file")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let json = read_json(infile);
        let set_of = json2set_m(&json);

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
