use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("refine")
        .about("Realign files with external programs and trim unwanted regions")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* List of msa:
    * mafft
    * muscle
    * clustalw

* For aligned files converted from .axt or .maf, we can use the `--quick` option
  to align only indel-adjacent regions

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
            Arg::new("msa")
                .long("msa")
                .num_args(1)
                .default_value("clustalw")
                .help("Aligning program"),
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
    let msa = args.get_one::<String>("msa").unwrap();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let mut seqs = vec![];
            let mut ranges = vec![];
            for entry in &block.entries {
                seqs.push(entry.seq().as_ref());
                ranges.push(entry.range().clone());
            }

            let aligned = align_seqs(&seqs, msa)?;

            //----------------------------
            // Output
            //----------------------------
            for (range, seq) in ranges.iter().zip(aligned) {
                // eprintln!("range = {:#?}", range.to_string());
                writer.write_all(format!(">{}\n{}\n", range, seq).as_ref())?;
            }

            // end of a block
            writer.write_all("\n".as_ref())?;
        }
    }

    Ok(())
}
