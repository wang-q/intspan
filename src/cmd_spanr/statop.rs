use clap::*;
use intspan::*;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("statop")
        .about("Coverage on chromosomes for one YAML crossed another")
        .after_help("Only the *first* file can contain multiple sets of runlists")
        .arg(
            Arg::new("chr.sizes")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("infile1")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("infile2")
                .help("Sets the input file to use")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Only write whole genome stats"),
        )
        .arg(
            Arg::new("op")
                .long("op")
                .takes_value(true)
                .default_value("intersect")
                .forbid_empty_values(true)
                .help("operations: intersect, union, diff or xor"),
        )
        .arg(
            Arg::new("base")
                .long("base")
                .takes_value(true)
                .help("basename of infile2"),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //----------------------------
    // Loading
    //----------------------------
    let sizes = read_sizes(args.value_of("chr.sizes").unwrap());

    let yaml: BTreeMap<String, Value> = read_yaml(args.value_of("infile1").unwrap());
    let is_multi: bool = yaml.values().next().unwrap().is_mapping();
    let mut s1_of = yaml2set_m(&yaml);

    let single: BTreeMap<String, Value> = read_yaml(args.value_of("infile2").unwrap());
    let mut s2 = yaml2set(&single);

    let is_all = args.is_present("all");
    let base = if args.is_present("base") {
        args.value_of("base").unwrap()
    } else {
        Path::new(args.value_of("infile2").unwrap())
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap()
    };
    let op = args.value_of("op").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    let chrs = sizes
        .keys()
        .map(|s| s.to_string())
        .collect::<BTreeSet<String>>();
    fill_up_m(&mut s1_of, &chrs);
    fill_up_s(&mut s2, &chrs);

    let mut res_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, s1) in &s1_of {
        let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in s1.keys() {
            let intspan = match op {
                "intersect" => s1.get(chr).unwrap().intersect(s2.get(chr).unwrap()),
                "diff" => s1.get(chr).unwrap().diff(s2.get(chr).unwrap()),
                "union" => s1.get(chr).unwrap().union(s2.get(chr).unwrap()),
                "xor" => s1.get(chr).unwrap().xor(s2.get(chr).unwrap()),
                _ => panic!("Invalid IntSpan Op"),
            };
            //            println!("Op {}: {}", op, op_intspan.to_string());
            set.insert(chr.into(), intspan);
        }
        res_of.insert(name.into(), set);
    }

    let mut lines: Vec<String> = Vec::new(); // Avoid lifetime problems
    let mut header = format!(
        "key,chr,chrLength,size,{}Length,{}Size,c1,c2,ratio",
        base, base
    );

    if is_multi {
        if is_all {
            header = header.replace("chr,", "");
        }
        lines.push(header);

        for name in s1_of.keys() {
            let key_lines = csv_lines(
                s1_of.get(name).unwrap(),
                &sizes,
                &s2,
                res_of.get(name).unwrap(),
                is_all,
                Some(name),
            );
            lines.push(key_lines);
        }
    } else {
        header = header.replace("key,", "");
        if is_all {
            header = header.replace("chr,", "");
        }
        lines.push(header);

        let key_lines = csv_lines(
            s1_of.get("__single").unwrap(),
            &sizes,
            &s2,
            res_of.get("__single").unwrap(),
            is_all,
            None,
        );
        lines.push(key_lines);
    }

    //----------------------------
    // Output
    //----------------------------
    write_lines(
        args.value_of("outfile").unwrap(),
        &lines.iter().map(AsRef::as_ref).collect(),
    )?;

    Ok(())
}

fn csv_lines(
    s1: &BTreeMap<String, IntSpan>,
    sizes: &BTreeMap<String, i32>,
    s2: &BTreeMap<String, IntSpan>,
    set_op: &BTreeMap<String, IntSpan>,
    is_all: bool,
    prefix: Option<&str>,
) -> String {
    let mut lines = String::new();

    let mut all_length: i64 = 0;
    let mut all_size: i64 = 0;
    let mut all_s2_length: i64 = 0;
    let mut all_s2_size: i64 = 0;
    for chr in s1.keys() {
        let length = *sizes.get(chr).unwrap();
        let size = s1.get(chr).unwrap().cardinality();

        let s2_length = s2.get(chr).unwrap().cardinality();
        let s2_size = set_op.get(chr).unwrap().cardinality();

        let c1 = size as f64 / length as f64;
        let c2 = if s2_length == 0 {
            0 as f64
        } else {
            s2_size as f64 / s2_length as f64
        };
        let ratio = if (c1 - 0 as f64).abs() < 0.00001 {
            0 as f64
        } else {
            c2 / c1
        };

        let line = format!(
            "{},{},{},{},{},{:.4},{:.4},{:.4}\n",
            chr, length, size, s2_length, s2_size, c1, c2, ratio
        );
        if let Some(s) = prefix {
            lines.push_str(format!("{},", s).as_str())
        };
        lines.push_str(line.as_str());

        all_length += length as i64;
        all_size += size as i64;
        all_s2_length += s2_length as i64;
        all_s2_size += s2_size as i64;
    }

    let all_c1 = all_size as f64 / all_length as f64;
    let all_c2 = if all_s2_length == 0 {
        0 as f64
    } else {
        all_s2_size as f64 / all_s2_length as f64
    };
    let all_ratio = if (all_c1 - 0 as f64).abs() < 0.00001 {
        0 as f64
    } else {
        all_c2 / all_c1
    };

    let mut all_line = format!(
        "{},{},{},{},{},{:.4},{:.4},{:.4}\n",
        "all", all_length, all_size, all_s2_length, all_s2_size, all_c1, all_c2, all_ratio
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
