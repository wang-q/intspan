use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("circos")
        .about("Convert links to circos links or highlights")
        .after_help(
            "\
             It's assumed that all ranges in input files are valid\
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
            Arg::with_name("highlight")
                .long("highlight")
                .help("Create highlights instead of links"),
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
    let is_highlight = args.is_present("highlight");

    let mut colors = (1..=12)
        .map(|n| format!("paired-12-qual-{}", n).to_string())
        .collect::<Vec<String>>();
    colors.reverse();
    let mut color_idx = 0;

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split('\t').collect();

            if is_highlight {
                for part in parts {
                    let range = Range::from_str(part);
                    if range.start() == &0 {
                        continue;
                    }

                    //----------------------------
                    // Output
                    //----------------------------
                    writer.write_all(
                        format!(
                            "{} {} {} fill_color={}\n",
                            range.chr(),
                            range.start(),
                            range.end(),
                            colors[color_idx]
                        )
                        .as_ref(),
                    );
                }

                // rotate color
                color_idx += 1;
                if color_idx > 11 {
                    color_idx = 0;
                }
            } else {
                let count = parts.len();

                // 2-combinations of parts forms a pair
                for i in 0..count {
                    'PAIR: for j in i + 1..count {
                        let mut fields: Vec<String> = vec![];
                        for idx in &[i, j] {
                            let range = Range::from_str(parts[*idx]);
                            if range.start() == &0 {
                                continue 'PAIR;
                            }

                            fields.push(range.chr().to_string());
                            if range.strand() == "-" {
                                fields.push(range.end().to_string());
                                fields.push(range.start().to_string());
                            } else {
                                fields.push(range.start().to_string());
                                fields.push(range.end().to_string());
                            }
                        }

                        //----------------------------
                        // Output
                        //----------------------------
                        writer.write_all(format!("{}\n", fields.join(" ")).as_ref());
                    }
                }
            }
        } // end of line
    }
}
