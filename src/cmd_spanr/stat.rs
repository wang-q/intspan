use clap::*;
use intspan::*;
use serde_json::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("stat")
        .about("Coverage on chromosomes for runlists")
        .arg(
            Arg::new("chr.sizes")
                .required(true)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .index(2)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Only write whole genome stats"),
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
    let sizes = read_sizes(args.get_one::<String>("chr.sizes").unwrap());

    let json: BTreeMap<String, Value> = read_json(args.get_one::<String>("infile").unwrap());
    let is_multi: bool = json.values().next().unwrap().is_object();

    let set_of = json2set_m(&json);

    let is_all = args.get_flag("all");

    //----------------------------
    // Operating
    //----------------------------
    let mut lines: Vec<String> = Vec::new(); // Avoid lifetime problems
    let mut header = "key,chr,chrLength,size,coverage".to_string();

    if is_multi {
        if is_all {
            header = header.replace("chr,", "");
        }
        lines.push(header);

        for (name, set) in &set_of {
            let key_lines = csv_lines(set, &sizes, is_all, Some(name));
            lines.push(key_lines);
        }
    } else {
        header = header.replace("key,", "");
        if is_all {
            header = header.replace("chr,", "");
        }
        lines.push(header);

        let key_lines = csv_lines(set_of.get("__single").unwrap(), &sizes, is_all, None);
        lines.push(key_lines);
    }

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.get_one::<String>("outfile").unwrap(),
        &lines.iter().map(AsRef::as_ref).collect(),
    )?;

    Ok(())
}

fn csv_lines(
    set: &BTreeMap<String, IntSpan>,
    sizes: &BTreeMap<String, i32>,
    is_all: bool,
    prefix: Option<&str>,
) -> String {
    let mut lines = String::new();

    let mut all_length: i64 = 0;
    let mut all_size: i64 = 0;
    for chr in set.keys() {
        let length = *sizes.get(chr).unwrap();
        let size = set.get(chr).unwrap().cardinality();
        let line = format!(
            "{},{},{},{:.4}\n",
            chr,
            length,
            size,
            size as f32 / length as f32
        );
        if let Some(s) = prefix {
            lines.push_str(format!("{},", s).as_str())
        };
        lines.push_str(line.as_str());

        all_length += length as i64;
        all_size += size as i64;
    }

    let mut all_line = format!(
        "{},{},{},{:.4}\n",
        "all",
        all_length,
        all_size,
        all_size as f64 / all_length as f64
    );
    // only keep whole genome
    if is_all {
        lines = String::new();
        all_line = all_line.replace("all,", "");
    }
    if let Some(s) = prefix {
        all_line.insert_str(0, format!("{},", s).as_str())
    };
    lines.push_str(all_line.as_str());

    // Remove last LF, as write_lines will append one
    lines.trim_end().to_string()
}
