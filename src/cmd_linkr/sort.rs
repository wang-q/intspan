use clap::*;
use intspan::*;
use std::collections::BTreeSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("sort")
        .about("Sort links and ranges within links")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
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
    let mut line_set: BTreeSet<String> = BTreeSet::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        'LINE: for line in reader.lines().map_while(Result::ok) {
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
    write_lines(args.get_one::<String>("outfile").unwrap(), &lines)?;

    Ok(())
}
