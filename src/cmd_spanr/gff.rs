use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("gff")
        .about("Convert gff3 to covers on chromosomes")
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .num_args(1)
                .help("primary tag (the third field)"),
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
    let mut res: BTreeMap<String, IntSpan> = BTreeMap::new();
    let tag = if args.contains_id("tag") {
        args.get_one::<String>("tag").unwrap().as_str()
    } else {
        ""
    };

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().map_while(Result::ok) {
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
    let out_json = set2json(&res);
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
