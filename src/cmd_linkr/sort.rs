use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::*;
use std::collections::BTreeSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("sort")
        .about("Sort links and ranges within links")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
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
    //----------------------------
    // Loading
    //----------------------------
    let mut line_set: BTreeSet<String> = BTreeSet::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split('\t').collect();

            for part in parts {
                let range = Range::from_str(part);
                if range.is_valid() {
                    line_set.insert(line.clone());
                    continue 'LINE;
                }
            }
        } // end of line
    }

    //----------------------------
    // Sorting
    //----------------------------
    let mut lines = line_set.into_iter().collect::<Vec<String>>();
    lines = sort_links(&lines);

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.value_of("outfile").unwrap(),
        &lines.iter().map(AsRef::as_ref).collect(),
    )?;

    Ok(())
}
