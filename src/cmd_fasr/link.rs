use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("link")
        .about("Output bi/multi-lateral range links")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
* infile == stdin means reading from STDIN

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
            Arg::new("pair")
                .long("pair")
                .action(ArgAction::SetTrue)
                .help("Bilateral (pairwise) links"),
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
    let is_pair = args.get_flag("pair");

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let mut headers = vec![];
            for entry in &block.entries {
                headers.push(entry.range().to_string());
            }

            //----------------------------
            // Output
            //----------------------------
            if is_pair {
                for i in 0..headers.len() {
                    for j in i+1..headers.len() {
                        writer.write_all(format!("{}\t{}\n", headers[i], headers[j]).as_ref())?;
                    }
                }
            } else {
                writer.write_all(format!("{}\n", headers.join("\t")).as_ref())?;
            }
        }
    }

    Ok(())
}
