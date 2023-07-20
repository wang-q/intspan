use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("axt2fas")
        .about("Convert axt to block fasta")
        .after_help(
            r###"
* <infiles> are paths to axt files, .axt.gz is supported
* infile == stdin means reading from STDIN

* We need the chr.sizes file for the query because without it we cannot compute
  the position on the negative strand

"###,
        )
        .arg(
            Arg::new("chr.sizes")
                .required(true)
                .index(1)
                .num_args(1)
                .help("The path to the query chr.sizes file"),
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(2)
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("tname")
                .long("tname")
                .num_args(1)
                .default_value("target")
                .help("Target name"),
        )
        .arg(
            Arg::new("qname")
                .long("qname")
                .num_args(1)
                .default_value("query")
                .help("Query name"),
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
    let mut writer = writer(args.get_one::<String>("outfile").unwrap());

    let sizes = read_sizes(args.get_one::<String>("chr.sizes").unwrap());

    let tname = args.get_one::<String>("tname").unwrap();
    let qname = args.get_one::<String>("qname").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_axt_block(&mut reader, &sizes, tname, qname) {
            for entry in block.entries {
                //----------------------------
                // Output
                //----------------------------
                writer.write_all(entry.to_string().as_ref())?;
            }

            // end of a block
            writer.write_all("\n".as_ref())?;
        }
    }

    Ok(())
}
