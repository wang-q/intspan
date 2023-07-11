use clap::*;
use intspan::*;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("circos")
        .about("Convert links to circos links or highlights")
        .after_help(
            r###"
* It's assumed that all ranges in input files are valid

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
            Arg::new("highlight")
                .long("highlight")
                .action(ArgAction::SetTrue)
                .help("Create highlights instead of links"),
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
    let is_highlight = args.get_flag("highlight");

    let mut colors = (1..=12)
        .map(|n| format!("paired-12-qual-{}", n))
        .collect::<Vec<String>>();
    colors.reverse();
    let mut color_idx = 0;

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().map_while(Result::ok) {
            let parts: Vec<&str> = line.split('\t').collect();

            if is_highlight {
                for part in parts {
                    let range = Range::from_str(part);
                    if !range.is_valid() {
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
                    )?;
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
                            if !range.is_valid() {
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
                        writer.write_all(format!("{}\n", fields.join(" ")).as_ref())?;
                    }
                }
            }
        } // end of line
    }

    Ok(())
}
