use clap::*;
use intspan::*;
use std::collections::HashSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("region")
        .about("Extract regions from a FA file")
        .after_help(
            "\
* region.txt contains fake runlists.
  There might be overlaps, e.g. I:17221-25234,21428-25459\
             ",
        )
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("region.txt")
                .help("seq_name:begin-end[,begin-end]")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .empty_values(false)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    Ok(())
}
