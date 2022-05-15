use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("cover")
        .about("Output covers on chromosomes")
        .after_help(
            r###"
Like command `combine`, but <infiles> are chromosome ranges

    I:1-100
    I(+):90-150             # Strand will be omitted
    S288c.I(-):190-200      # Species name will be omitted

"###,
        )
        .arg(
            Arg::new("infiles")
                .help("Sets the input file to use")
                .required(true)
                .min_values(1)
                .index(1),
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

    // seq_name => IntSpan
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

    for infile in args.values_of("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let range = Range::from_str(&line);
            if !range.is_valid() {
                continue;
            }
            let chr = range.chr();
            if !set.contains_key(chr) {
                set.insert(chr.clone(), IntSpan::new());
            }

            set.entry(chr.to_string())
                .and_modify(|e| e.add_pair(*range.start(), *range.end()));
        }
    }

    //----------------------------
    // Output
    //----------------------------
    let out_yaml = set2yaml(&set);
    write_yaml(args.value_of("outfile").unwrap(), &out_yaml)?;

    Ok(())
}
