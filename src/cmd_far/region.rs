use bio::io::fasta;
use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("region")
        .about("Extract regions from a FA file")
        .after_help(
            "\
* region.txt contains fake runlists.
  There might be overlaps, e.g. I:17221-25234,21428-25459\
             ",
        )
        .arg(
            Arg::new("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("region.txt")
                .help("seq_name:begin-end[,begin-end]")
                .required(false)
                .index(2),
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
    let reader = reader(args.value_of("infile").unwrap());

    let fa_in = fasta::Reader::new(reader);

    let mut n = 0;
    let mut sum = 0;
    for result in fa_in.records() {
        // obtain record or fail with error
        let record = result.unwrap();
        // obtain sequence
        let seq = record.seq();
        sum += seq.len();
        n += 1;
    }
    println!(
        "mean sequence length of {} records: {:.1} bp",
        n,
        sum as f32 / n as f32
    );

    Ok(())
}
