use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("consensus")
        .about("Generate consensus sequences by POA")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
* infile == stdin means reading from STDIN

* Need `spoa` in $PATH
* The original `poa` was unstable and sometimes crashed

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("cname")
                .long("cname")
                .num_args(1)
                .default_value("consensus")
                .help("Consensus name"),
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
    let cname = args.get_one::<String>("cname").unwrap();

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let mut seqs = vec![];
            for entry in &block.entries {
                seqs.push(entry.seq().as_ref());
            }

            let mut cons = get_consensus_poa(&seqs).unwrap();
            cons = cons.replace('-', "");

            let mut range = block.entries.first().unwrap().range().clone();

            //----------------------------
            // Output
            //----------------------------
            if range.is_valid() {
                *range.name_mut() = cname.to_string();
                writer.write_all(format!(">{}\n{}\n", range, cons).as_ref())?;
            } else {
                writer.write_all(format!(">{}\n{}\n", cname, cons).as_ref())?;
            }

            // end of a block
            writer.write_all("\n".as_ref())?;
        }
    }

    Ok(())
}
