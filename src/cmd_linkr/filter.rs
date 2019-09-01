use clap::*;
use intspan::*;
use std::collections::BTreeSet;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("filter")
        .about("Filter links by numbers of ranges or length differences")
        .after_help(
            "\
             It's assumed that all ranges in input files are valid.
             Inputs should not contain hit strands.\
             ",
        )
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .takes_value(true)
                .help("Numbers of ranges, an IntSpan like [2-10]"),
        )
        .arg(
            Arg::with_name("ratio")
                .long("ratio")
                .short("r")
                .takes_value(true)
                .help("Ratio of lengths differences. The suggested value is [0.8]"),
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
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let mut writer = writer(args.value_of("outfile").unwrap());

    let numbers = if args.is_present("number") {
        IntSpan::from(args.value_of("number").unwrap().to_string())
    } else {
        IntSpan::new()
    };
    let ratio: f32 = if args.is_present("ratio") {
        value_t!(args.value_of("ratio"), f32).unwrap_or_else(|e| {
            eprintln!("Need a float for --ratio\n{}", e);
            std::process::exit(1)
        })
    } else {
        -1.0
    };

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split('\t').collect();

            if !numbers.is_empty() {
                if !numbers.contains(parts.len() as i32) {
                    continue;
                }
            }

            if ratio != -1.0 {
                let mut lengths: Vec<i32> = vec![];

                for part in &parts {
                    let range = Range::from_str(*part);
                    if !range.is_valid() {
                        continue;
                    }
                    lengths.push(range.intspan().cardinality());
                }

                let min = lengths.iter().min().unwrap();
                let max = lengths.iter().max().unwrap();
                let diff_ratio = *min as f32 / *max as f32;

                if diff_ratio < ratio {
                    continue;
                }
            }

            //----------------------------
            // Output
            //----------------------------
            writer.write_all(format!("{}\n", line).as_ref());
        } // end of line
    }
}
