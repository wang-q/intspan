use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("convert")
        .about("Convert runlist file to ranges file")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("longest")
                .long("longest")
                .action(ArgAction::SetTrue)
                .help("Only keep the longest range"),
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
    // Args
    //----------------------------
    let is_longest = args.get_flag("longest");
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let json = intspan::read_json(infile);
        let set_of = intspan::json2set_m(&json);

        for set in set_of.values() {
            for chr in set.keys() {
                let ints = set.get(chr).unwrap();
                let mut intses = ints.intses();

                //----------------------------
                // Output
                //----------------------------
                if is_longest {
                    if !intses.is_empty() {
                        // Negate the value for descending order
                        intses.sort_by_cached_key(|e| -e.size());
                        let longest = intses.first().unwrap();
                        writer.write_all(format!("{}:{}\n", chr, longest).as_ref())?;
                    }
                } else {
                    for sub in &intses {
                        writer.write_all(format!("{}:{}\n", chr, sub).as_ref())?;
                    }
                }
            }
        }
    }

    Ok(())
}
