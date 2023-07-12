use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
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
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Sets the input files to use"),
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

    // seq_name => IntSpan
    let mut set: BTreeMap<String, IntSpan> = BTreeMap::new();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let reader = reader(infile);
        for line in reader.lines().map_while(Result::ok) {
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
    let out_json = set2json(&set);
    write_json(args.get_one::<String>("outfile").unwrap(), &out_json)?;

    Ok(())
}
