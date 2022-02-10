use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// TODO: optional chr.sizes to be passed to Coverage::new()

// TODO: improve speeds on large files

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> App<'a> {
    App::new("coverage")
        .about("Output detailed depths of coverages on chromosomes")
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("coverage")
                .help("minimal coverage")
                .long("coverage")
                .short('c')
                .takes_value(true)
                .default_value("1")
                .forbid_empty_values(true),
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
pub fn execute(args: &ArgMatches) -> std::result::Result<(), std::io::Error> {
    //----------------------------
    // Loading
    //----------------------------
    let coverage: i32 = args.value_of_t("coverage").unwrap_or_else(|e| {
        eprintln!("Need a integer for --coverage\n{}", e);
        std::process::exit(1)
    });

    // seq_name => tier_of => IntSpan
    let mut res: BTreeMap<String, Coverage> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let range = Range::from_str(&line);
            if !range.is_valid() {
                continue;
            }
            let chr = range.chr();
            if !res.contains_key(chr) {
                let tiers = Coverage::new(coverage);
                res.insert(chr.clone(), tiers);
            }

            res.entry(chr.to_string())
                .and_modify(|e| e.bump(range.start().clone(), range.end().clone()));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();
    for chr in res.keys() {
        set.insert(chr.to_string(), res.get(chr).unwrap().max_tier());
    }
    let out_yaml = set2yaml(&set);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;

    Ok(())
}
