use crate::utils::*;
use clap::*;
use intspan::IntSpan;
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("statop")
        .about("Coverage on chromosomes for one YAML crossed another\nOnly the *first* file can contain multiple sets of runlists")
        .arg(
            Arg::with_name("chr.sizes")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("infile1")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("infile2")
                .help("Sets the input file to use")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("all")
                .long("all")
                .help("Only write whole genome stats"),
        )
        .arg(
            Arg::with_name("op")
                .long("op")
                .takes_value(true)
                .default_value("intersect")
                .empty_values(false)
                .help("operations: intersect, union, diff or xor"),
        )
        .arg(
            Arg::with_name("base")
                .long("base")
                .takes_value(true)
                .help("basename of infile2"),
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

// test command implementation
pub fn execute(args: &ArgMatches) {
    //----------------------------
    // Loading
    //----------------------------
    let length_of = read_sizes(args.value_of("chr.sizes").unwrap());

    let master: BTreeMap<String, Value> = read_runlist(args.value_of("infile1").unwrap());
    let is_mk: bool = master.values().next().unwrap().is_mapping();
    let mut s1_of = to_set_of(&master);

    let single: BTreeMap<String, Value> = read_runlist(args.value_of("infile2").unwrap());
    let mut s2 = runlist2set(&single);

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
    fill_up(&mut s1_of, &length_of);
    fill_up_s(&mut s2, &length_of);

    let mut op_result_of: BTreeMap<String, BTreeMap<String, IntSpan>> = BTreeMap::new();
    for (name, s1) in &s1_of {
        let mut set_op: BTreeMap<String, IntSpan> = BTreeMap::new();
        for chr in s1.keys() {
            let intspan_op = match op {
                "intersect" => s1.get(chr).unwrap().intersect(s2.get(chr).unwrap()),
                "diff" => s1.get(chr).unwrap().diff(s2.get(chr).unwrap()),
                "union" => s1.get(chr).unwrap().union(s2.get(chr).unwrap()),
                "xor" => s1.get(chr).unwrap().xor(s2.get(chr).unwrap()),
                _ => panic!("Invalid IntSpan Op"),
            };
            //            println!("Op {}: {}", op, op_intspan.to_string());
            set_op.insert(chr.into(), intspan_op);
        }
        op_result_of.insert(name.into(), set_op);
    }

    let mut lines: Vec<String> = Vec::new(); // Avoid lifetime problems
    let mut header = format!(
        "key,chr,chrLength,size,{}Length,{}Size,c1,c2,ratio",
        base, base
    );

    if is_mk {
        if is_all {
            header = header.replace("chr,", "");
        }
        lines.push(header);

        for name in s1_of.keys() {
            let key_lines = csv_lines(
                s1_of.get(name).unwrap(),
                &length_of,
                &s2,
                op_result_of.get(name).unwrap(),
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
            &length_of,
            &s2,
            op_result_of.get("__single").unwrap(),
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
    );
}

fn csv_lines(
    set_one: &BTreeMap<String, IntSpan>,
    length_of: &BTreeMap<String, i32>,
    s2: &BTreeMap<String, IntSpan>,
    set_op: &BTreeMap<String, IntSpan>,
    is_all: bool,
    prefix: Option<&str>,
) -> String {
    let mut lines = String::new();

    let mut all_length = 0;
    let mut all_size = 0;
    let mut all_s2_length = 0;
    let mut all_s2_size = 0;
    for chr in set_one.keys() {
        let length = *length_of.get(chr).unwrap();
        let size = set_one.get(chr).unwrap().cardinality();

        let s2_length = s2.get(chr).unwrap().cardinality();
        let s2_size = set_op.get(chr).unwrap().cardinality();

        let c1 = size as f32 / length as f32;
        let c2 = if s2_length == 0 {
            0 as f32
        } else {
            s2_size as f32 / s2_length as f32
        };
        let ratio = if c1 == 0 as f32 { 0 as f32 } else { c2 / c1 };

        let line = format!(
            "{},{},{},{},{},{:.4},{:.4},{:.4}\n",
            chr, length, size, s2_length, s2_size, c1, c2, ratio
        );
        match prefix {
            Some(s) => lines.push_str(format!("{},", s).as_str()),
            None => (),
        };
        lines.push_str(line.as_str());

        all_length += length;
        all_size += size;
        all_s2_length += s2_length;
        all_s2_size += s2_size;
    }

    let all_c1 = all_size as f32 / all_length as f32;
    let all_c2 = if all_s2_length == 0 {
        0 as f32
    } else {
        all_s2_size as f32 / all_s2_length as f32
    };
    let all_ratio = if all_c1 == 0 as f32 {
        0 as f32
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
    match prefix {
        Some(s) => all_line.insert_str(0, format!("{},", s).as_str()),
        None => (),
    };
    lines.push_str(all_line.as_str());

    // Remove last LF, as write_lines will append one
    lines.trim_end().to_string()
}
