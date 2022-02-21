use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("size")
        .about("Count total bases in FA file(s)")
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
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    let mut writer = writer(args.value_of("outfile").unwrap());

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        let fa_in = fasta::Reader::new(reader);

        for result in fa_in.records() {
            // obtain record or fail with error
            let record = result.unwrap();

            writer.write_fmt(format_args!("{}\t{}\n", record.id(), record.seq().len()))?;
        }
    }

    Ok(())
}
