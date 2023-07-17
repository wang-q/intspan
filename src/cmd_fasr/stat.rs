use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("stat")
        .about("Extract a subset of species")
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
            Arg::new("outgroup")
                .long("outgroup")
                .action(ArgAction::SetTrue)
                .help("Alignments have an outgroup'"),
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
    let is_outgroup = args.get_flag("outgroup");

    let field_names = vec![
        "target",
        "length",
        "comparable",
        "difference",
        "gap",
        "ambiguous",
        "D",
        "indel",
    ];

    //----------------------------
    // Operating
    //----------------------------
    writer.write_all(format!("{}\n", field_names.join("\t")).as_ref())?;

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let target = block.entries.first().unwrap().range().to_string();

            let mut seqs = vec![];
            for entry in &block.entries {
                seqs.push(entry.seq().as_ref());
            }

            if is_outgroup {
                seqs.pop();
            }

            let (length, comparable, difference, gap, ambiguous, mean_d) = alignment_stat(&seqs);

            let mut indel_ints = IntSpan::new();
            for seq in seqs {
                indel_ints.merge(&indel_intspan(seq));
            }

            writer.write_all(
                format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    target,
                    length,
                    comparable,
                    difference,
                    gap,
                    ambiguous,
                    mean_d,
                    indel_ints.span_size(),
                )
                .as_ref(),
            )?;
        }
    }

    Ok(())
}
