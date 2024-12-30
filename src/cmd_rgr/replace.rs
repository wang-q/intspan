use clap::*;
use std::collections::HashMap;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("replace")
        .about("Replace fields in a .tsv file using a replacement map")
        .after_help(
            r###"
Examples:

    # Replace fields
    rgr replace tests/rgr/1_4.ovlp.tsv tests/rgr/1_4.replace.tsv

    # Reverse the replacement map (To--From instead of From--To)
    rgr replace tests/rgr/1_4.ovlp.tsv tests/rgr/1_4.replace.tsv -r

"###,
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .num_args(1)
                .index(1)
                .help("Input file to process"),
        )
        .arg(
            Arg::new("replace")
                .required(true)
                .num_args(1)
                .index(2)
                .help("Replacement map file with two columns: From and To"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .short('r')
                .action(ArgAction::SetTrue)
                .help("Use the replacement map in reverse order (To--From instead of From--To)"),
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
    // Args
    //----------------------------
    let mut writer = intspan::writer(args.get_one::<String>("outfile").unwrap());
    let reader = intspan::reader(args.get_one::<String>("infile").unwrap());

    //----------------------------
    // Load replacements
    //----------------------------
    let mut replaces: HashMap<String, String> = HashMap::new();
    for line in intspan::read_lines(args.get_one::<String>("replace").unwrap()) {
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
            if let Some(replacement) = replaces.get(f) {
                out.push(replacement);
            } else {
                out.push(f);
            }
        }

        writer.write_all((out.join("\t") + "\n").as_ref())?;
    }

    Ok(())
}
