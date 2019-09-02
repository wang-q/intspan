use clap::*;
use intspan::*;
use serde_yaml::Value;
use std::collections::BTreeMap;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("stat")
        .about("Coverage on chromosomes for runlists")
        .arg(
            Arg::with_name("chr.sizes")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("all")
                .long("all")
                .help("Only write whole genome stats"),
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
    let sizes = read_sizes(args.value_of("chr.sizes").unwrap());

    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile").unwrap());
    let is_multi: bool = yaml.values().next().unwrap().is_mapping();

    let set_of = yaml2set_m(&yaml);

    let is_all = args.is_present("all");

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
        args.value_of("outfile").unwrap(),
        &lines.iter().map(AsRef::as_ref).collect(),
    );
}

fn csv_lines(
    set: &BTreeMap<String, IntSpan>,
    sizes: &BTreeMap<String, i32>,
    is_all: bool,
    prefix: Option<&str>,
) -> String {
    let mut lines = String::new();

    let mut all_length = 0;
    let mut all_size = 0;
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

        all_length += length;
        all_size += size;
    }

    let mut all_line = format!(
        "{},{},{},{:.4}\n",
        "all",
        all_length,
        all_size,
        all_size as f32 / all_length as f32
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
