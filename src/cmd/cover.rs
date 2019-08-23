use crate::utils::*;
use clap::{App, Arg, ArgMatches, SubCommand};
use intspan::{IntSpan, Range};
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("cover")
        .about("Output covers on chromosomes")
        .after_help(
            "\
Like command `combine`, but <infiles> are chromosome ranges

    I:1-100
    I(+):90-150             # Strand will be omitted
    S288c.I(-):190-200      # Species name will be omitted \
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
    let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let range = Range::from_str(line);
            if range.start() == &0 {
                continue;
            }
            let chr = range.chr();
            if !res.contains_key(chr) {
                let intspan = IntSpan::new();
                res.insert(chr.clone(), intspan);
            }
            res.entry(chr.to_string())
                .and_modify(|e| e.add_pair(range.start().clone(), range.end().clone()));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_yaml = set2yaml(&res);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml);
}
