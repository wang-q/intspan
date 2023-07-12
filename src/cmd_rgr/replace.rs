use clap::*;
use intspan::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("replace")
        .about("Replace fields in .tsv file")
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Sets the input file to use"),
        )
        .arg(
            Arg::new("replace")
                .required(true)
                .num_args(1)
                .index(2)
                .help("A two-column tsv file, From--To"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('r')
                .action(ArgAction::SetTrue)
                .help("To--From instead in .replace.tsv"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());
    let reader = reader(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Load replaces
    //----------------------------
    let mut replaces: HashMap<String, String> = HashMap::new();
    for line in read_lines(args.get_one::<String>("replace").unwrap()) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            if args.get_flag("reverse") {
                replaces.insert(parts[1].to_string(), parts[0].to_string());
            } else {
                replaces.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    //----------------------------
    // Output
    //----------------------------
    for line in reader.lines().map_while(Result::ok) {
        let fields: Vec<&str> = line.split('\t').collect();
        let mut out: Vec<&str> = vec![];

        for f in fields {
            if replaces.contains_key(f) {
                out.push(replaces.get(f).unwrap());
            } else {
                out.push(f);
            }
        }

        writer.write_all((out.join("\t") + "\n").as_ref())?;
    }

    Ok(())
}
