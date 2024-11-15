use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("md")
        .about("Convert .tsv file to markdown table")
        .after_help(
            r###"
* --right 1,3-5

* Using `--fmt --digits 2` will produce the output in the format `1,234.00`.

"###,
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("center")
                .long("center")
                .short('c')
                .num_args(1)
                .help("Center-aligned columns"),
        )
        .arg(
            Arg::new("right")
                .long("right")
                .short('r')
                .num_args(1)
                .help("Right-aligned columns"),
        )
        .arg(
            Arg::new("num")
                .long("num")
                .action(ArgAction::SetTrue)
                .help("Right-aligning numeric columns"),
        )
        .arg(
            Arg::new("fmt")
                .long("fmt")
                .action(ArgAction::SetTrue)
                .help("Format numeric columns and enable the `--num` option"),
        )
        .arg(
            Arg::new("digits")
                .long("digits")
                .num_args(1)
                .default_value("0")
                .value_parser(value_parser!(usize))
                .help("Decimal digits"),
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
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());
    let reader = intspan::reader(args.get_one::<String>("infile").unwrap());

    let mut opt_center: intspan::IntSpan = if args.contains_id("center") {
        intspan::fields_to_ints(args.get_one::<String>("center").unwrap())
    } else {
        intspan::IntSpan::new()
    };
    let mut opt_right: intspan::IntSpan = if args.contains_id("right") {
        intspan::fields_to_ints(args.get_one::<String>("right").unwrap())
    } else {
        intspan::IntSpan::new()
    };
    let mut is_num = args.get_flag("num");
    let is_fmt = args.get_flag("fmt");
    if is_fmt {
        is_num = true;
    }
    let opt_digits: usize = *args.get_one("digits").unwrap();

    //----------------------------
    // Output
    //----------------------------
    let mut is_numeric_column = vec![];

    let mut data: Vec<Vec<String>> = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let fields: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();
        data.push(fields);
    }

    if !data.is_empty() {
        let num_columns = data[0].len();
        if is_num {
            // Determine if each column is numeric
            is_numeric_column = vec![true; num_columns];

            for row in data.iter().skip(1) {
                // Skip the header row
                for (i, value) in row.iter().enumerate() {
                    if is_numeric_column[i] && value.parse::<f64>().is_err() {
                        is_numeric_column[i] = false;
                    }
                }
            }

            for i in 0..num_columns {
                if is_numeric_column[i] {
                    opt_center.remove_n((i + 1) as i32);
                    opt_right.add_n((i + 1) as i32);
                }
            }
        }

        // Print the Markdown table
        for (i, row) in data.iter().enumerate() {
            let formatted_row: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(j, value)| {
                    // Don't touch first row
                    if i == 0 {
                        format!("{}", value)
                    } else if is_fmt && is_numeric_column[j] {
                        let num = value.parse::<f64>().unwrap();
                        let v = intspan::format_number(num, opt_digits);
                        format!("{}", v)
                    } else {
                        format!("{}", value)
                    }
                })
                .collect();
            writer.write_fmt(format_args!("| {} |\n", formatted_row.join(" | ")))?;

            // Print the header separator
            if i == 0 {
                let separator: Vec<String> = (0..num_columns)
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|&j| {
                        if opt_right.contains((j + 1) as i32) {
                            "---:".to_string()
                        } else if opt_center.contains((j + 1) as i32) {
                            ":---:".to_string()
                        } else {
                            "---".to_string()
                        }
                    })
                    .collect();
                writer.write_fmt(format_args!("| {} |\n", separator.join(" | ")))?;
            }
        }
    }

    Ok(())
}
