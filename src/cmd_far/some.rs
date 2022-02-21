use bio::io::fasta;
use clap::*;
use intspan::*;
use std::collections::HashSet;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("some")
        .about("Extract some FA records")
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("lst")
                .help("One name per line")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .short('i')
                .help("Output sequences not in the list"),
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
    let is_invert = args.is_present("invert");

    let reader = reader(args.value_of("infile").unwrap());
    let fa_in = fasta::Reader::new(reader);

    let writer = writer(args.value_of("outfile").unwrap());
    let mut fa_out = fasta::Writer::new(writer);

    let set_lst: HashSet<String> = read_first_column(args.value_of("lst").unwrap())
        .into_iter()
        .collect();

    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();

        if set_lst.contains(record.id()) != is_invert {
            fa_out.write_record(&record).expect("Error writing record.");
        }
    }

    Ok(())
}
