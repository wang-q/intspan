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
        to_ints(args.get_one::<String>("center").unwrap())
    } else {
        intspan::IntSpan::new()
    };
    let mut opt_right: intspan::IntSpan = if args.contains_id("right") {
        to_ints(args.get_one::<String>("right").unwrap())
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
                    if i == 0  {
                        format!("{}", value)
                    } else if is_fmt && is_numeric_column[j] {
                        let num = value.parse::<f64>().unwrap();
                        let v = format_number(num, opt_digits);
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

fn to_ints(str: &str) -> intspan::IntSpan {
    let mut ints = intspan::IntSpan::new();
    let parts: Vec<&str> = str.split(',').collect();
    for p in parts {
        ints.add_runlist(p);
    }
    ints
}

// rewrite from https://metacpan.org/dist/Number-Format/source/Format.pm
fn format_number(number: f64, decimal_digits: usize) -> String {
    // Handle negative numbers
    let sign = if number < 0.0 { -1 } else { 1 };
    let mut number = number.abs();
    number = round(number, decimal_digits); // Round off number

    // Split integer and decimal parts of the number
    let integer_part = number.trunc() as i64;
    let decimal_part = number.fract();

    // Add the commas (fixed as `,`)
    let integer_str = integer_part.to_string();
    let formatted_integer = integer_str
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();

    let decimal_str = format!("{:.1$}", decimal_part, decimal_digits)
        .trim_start_matches('0')
        .to_string();

    let result = if !decimal_str.is_empty() {
        format!("{}{}", formatted_integer, decimal_str)
    } else {
        formatted_integer
    };

    if sign < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

fn round(number: f64, precision: usize) -> f64 {
    // Implement rounding logic
    (number * 10f64.powi(precision as i32)).round() / 10f64.powi(precision as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        // Test positive numbers
        assert_eq!(format_number(1234567.89, 2), "1,234,567.89");
        assert_eq!(format_number(1000.0, 0), "1,000");
        assert_eq!(format_number(0.12345, 3), "0.123");

        // Test negative numbers
        assert_eq!(format_number(-9876543.21, 3), "-9,876,543.210");
        assert_eq!(format_number(-1000.0, 0), "-1,000");
        assert_eq!(format_number(-0.98765, 4), "-0.9877");

        // Test zero
        assert_eq!(format_number(0.0, 2), "0.00");
        assert_eq!(format_number(-0.0, 2), "0.00");

        // Test large numbers
        assert_eq!(format_number(1e10, 2), "10,000,000,000.00");
        assert_eq!(format_number(-1e10, 2), "-10,000,000,000.00");

        // Test decimal places
        assert_eq!(format_number(1234.56789, 3), "1,234.568");
        assert_eq!(format_number(1234.0, 5), "1,234.00000");
    }
}
