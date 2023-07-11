use clap::*;
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("filter")
        .about("Filter links by numbers of ranges or length differences")
        .after_help(
            r###"
* It's assumed that all ranges in input files are valid
* Inputs should not contain hit strands

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
        )
        .arg(
            Arg::new("number")
                .long("number")
                .short('n')
                .num_args(1)
                .help("Numbers of ranges, an IntSpan like [2-10]"),
        )
        .arg(
            Arg::new("ratio")
                .long("ratio")
                .short('r')
                .num_args(1)
                .value_parser(value_parser!(f32))
                .help("Ratio of lengths differences. The suggested value is [0.8]"),
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

    let numbers = if args.contains_id("number") {
        IntSpan::from(args.get_one::<String>("number").unwrap())
    } else {
        IntSpan::new()
    };
    let ratio = if args.contains_id("ratio") {
        *args.get_one::<f32>("ratio").unwrap()
    } else {
        -1.0
    };

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split('\t').collect();

            if !numbers.is_empty() && !numbers.contains(parts.len() as i32) {
                continue;
            }

            if ratio > 0.0 {
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
            writer.write_all(format!("{}\n", line).as_ref())?;
        } // end of line
    }

    Ok(())
}
