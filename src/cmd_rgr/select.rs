use clap::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("select")
        .about("Select fields in the order listed")
        .after_help(
            r###"
* Fields can be specified by field number or field name.
* Field names must not be specified as a valid IntSpan runlist.
  For example, avoid using formats like `1`, `2-6`, or `-`.

Examples:

    rgr select tests/rgr/ctg.tsv -H -f 6,1

    rgr select tests/rgr/ctg.tsv -H -f ID,length

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .short('H')
                .action(ArgAction::SetTrue)
                .help("Treat the first line of each file as a header"),
        )
        .arg(
            Arg::new("sharp")
                .long("sharp")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Write the lines starting with a `#` without changes. The default is to ignore them"),
        )
        .arg(
            Arg::new("fields")
                .long("fields")
                .short('f')
                .num_args(1)
                .help("Writes selected fields and the generated range field, in the order listed"),
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
    // Args
    //----------------------------
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());

    let is_header = args.get_flag("header");
    let is_sharp = args.get_flag("sharp");

    //----------------------------
    // Ops
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = intspan::reader(infile);
        let mut fields: Vec<usize> = vec![];

        'LINE: for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
            let parts: Vec<&str> = line.split('\t').collect();

            // the header line
            if i == 0 {
                if is_header {
                    let mut idx_of: HashMap<String, usize> = HashMap::new();
                    for (i, field) in parts.iter().enumerate() {
                        idx_of.insert(field.to_string(), i + 1);
                    }

                    if args.contains_id("fields") {
                        fields = intspan::named_field_to_idx(
                            args.get_one::<String>("fields").unwrap(),
                            &idx_of,
                        )
                        .unwrap()
                    };
                } else if args.contains_id("fields") {
                    fields = intspan::fields_to_idx(args.get_one::<String>("fields").unwrap());
                }

                if fields.is_empty() {
                    writer.write_fmt(format_args!("{}\n", line))?;
                } else {
                    let selected: Vec<String> = fields
                        .iter()
                        .map(|e| parts.get(*e - 1).unwrap().to_string())
                        .collect();

                    writer.write_fmt(format_args!("{}\n", selected.join("\t")))?;
                }
                continue 'LINE;
            }

            if line.starts_with('#') {
                if is_sharp {
                    writer.write_fmt(format_args!("{}\n", line))?;
                }
                continue 'LINE;
            }

            //----------------------------
            // Output
            //----------------------------
            let new_line: String = if fields.is_empty() {
                parts.join("\t").to_string()
            } else {
                let selected: Vec<String> = fields
                    .iter()
                    .map(|e| parts.get(*e - 1).unwrap().to_string())
                    .collect();

                selected.join("\t")
            };

            writer.write_fmt(format_args!("{}\n", new_line))?;
        }
    }

    Ok(())
}
