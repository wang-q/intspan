use clap::*;
use intspan::*;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("variation")
        .about("List variations (substitutions/indels)")
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
                .help("Set the input files to use"),
        )
        .arg(
            Arg::new("indel")
                .long("indel")
                .action(ArgAction::SetTrue)
                .help("List indels"),
        )
        .arg(
            Arg::new("has_outgroup")
                .long("outgroup")
                .action(ArgAction::SetTrue)
                .help("There are outgroups at the end of each block"),
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
    let has_outgroup = args.get_flag("has_outgroup");

    let field_names = vec![
        "#target",
        "chr",
        "pos",
        "chr_pos",
        "range",
        "tbase",
        "qbase",
        "bases",
        "mutant_to",
        "freq",
        "pattern",
        "obase",
    ];

    //----------------------------
    // Operating
    //----------------------------
    writer.write_all(format!("{}\n", field_names.join("\t")).as_ref())?;

    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let mut seqs: Vec<&[u8]> = vec![];
            for entry in &block.entries {
                seqs.push(entry.seq().as_ref());
            }

            // target range and sequence intspan
            let trange = block.entries.first().unwrap().range().clone();
            let t_ints_seq = seq_intspan(block.entries.first().unwrap().seq());

            // pos, tbase, qbase, bases, mutant_to, freq, pattern, obase
            //   0,     1,     2,     3,         4,    5,       6,     7
            let seq_count = seqs.len();
            let subs = if has_outgroup {
                let mut unpolarized = get_subs(&seqs[..(seq_count - 1)]).unwrap();
                polarize_subs(&mut unpolarized, seqs[seq_count - 1]);
                unpolarized
            } else {
                get_subs(&seqs).unwrap()
            };

            for s in subs {
                let chr = trange.chr();

                let chr_pos =
                    align_to_chr(&t_ints_seq, s.pos, trange.start, trange.strand()).unwrap();
                let var_rg = format!("{}:{}", chr, chr_pos);

                writer.write_all(
                    format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                        trange,
                        chr,
                        s.pos,
                        chr_pos,
                        var_rg,
                        s.tbase,
                        s.qbase,
                        s.bases,
                        s.mutant_to,
                        s.freq,
                        s.pattern,
                        s.obase,
                    )
                    .as_ref(),
                )?;
            }
        }
    }

    Ok(())
}
