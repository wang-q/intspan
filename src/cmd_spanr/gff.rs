use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("gff")
        .about("Convert gff3 to covers on chromosomes")
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .takes_value(true)
                .help("primary tag (the third field)"),
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
    let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
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
            if !tag.is_empty() && feature != tag {
                continue;
            }

            let chr = fields[0];
            let start = fields[3].parse::<i32>().unwrap();
            let end = fields[4].parse::<i32>().unwrap();

            if !res.contains_key(chr) {
                let intspan = IntSpan::new();
                res.insert(chr.to_string(), intspan);
            }
            res.entry(chr.to_string())
                .and_modify(|e| e.add_pair(start, end));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_yaml = set2yaml(&res);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;

    Ok(())
}
