use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::{IntSpan, Range};
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("gff")
        .about("Convert gff3 to covers on chromosomes")
        .arg(
            Arg::with_name("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::with_name("tag")
                .long("tag")
                .takes_value(true)
                .help("primary tag (the third field)"),
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
    let mut covered: BTreeMap<String, IntSpan> = BTreeMap::new();
    let tag = if args.is_present("tag") {
        args.value_of("tag").unwrap()
    } else {
        ""
    };

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            if line.starts_with('#') {
                continue;
            }

            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() < 8 {
                continue;
            }

            let feature = fields[2];
            if tag.len() > 0 {
                if feature != tag {
                    continue;
                }
            }

            let chr = fields[0];
            let start = fields[3].parse::<i32>().unwrap();
            let end = fields[4].parse::<i32>().unwrap();

            if !covered.contains_key(chr) {
                let intspan = IntSpan::new();
                covered.insert(chr.to_string(), intspan);
            }
            covered
                .entry(chr.to_string())
                .and_modify(|e| e.add_pair(start, end));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_runlist = set2runlist(&covered);
    write_runlist(args.value_of("outfile").unwrap(), &out_runlist);
}
