use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("separate")
        .about("Separate block fasta files by species")
        .after_help(
            r###"
* <infiles> are paths to block fasta files, .fas.gz is supported
    * infile == stdin means reading from STDIN

* Dashes ('-') will be removed from sequences

* If the target file exists, it will be overwritten

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
            Arg::new("suffix")
                .long("suffix")
                .short('s')
                .num_args(1)
                .default_value(".fasta")
                .help("Extensions of output files"),
        )
        .arg(
            Arg::new("rc")
                .long("rc")
                .action(ArgAction::SetTrue)
                .help("Revcom sequences when chr_strand is '-'"),
        )
        .arg(
            Arg::new("outdir")
                .short('o')
                .long("outdir")
                .num_args(1)
                .default_value("stdout")
                .help("Output location. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let outdir = args.get_one::<String>("outdir").unwrap();
    if outdir != "stdout" {
        fs::create_dir_all(outdir)?;
    }

    let suffix = args.get_one::<String>("suffix").unwrap();
    let is_rc = args.get_flag("rc");

    let mut file_of: BTreeMap<String, File> = BTreeMap::new();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            for entry in &block.entries {
                let entry_name = entry.range().name(); // Don't borrow the following `range`
                let mut range = entry.range().clone();

                let seq = if is_rc && range.strand() == "-" {
                    *range.strand_mut() = "+".to_string();
                    bio::alphabets::dna::revcomp(entry.seq())
                } else {
                    entry.seq().to_vec()
                };
                let seq = std::str::from_utf8(&seq)
                    .unwrap()
                    .to_string()
                    .replace('-', "");

                //----------------------------
                // Output
                //----------------------------
                if outdir == "stdout" {
                    print!(">{}\n{}\n", range, seq);
                } else {
                    if !file_of.contains_key(entry_name) {
                        let path = Path::new(outdir).join(range.name().to_owned() + suffix);
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(path)?;
                        file_of.insert(entry_name.to_string(), file);
                    }
                    write!(file_of.get(entry_name).unwrap(), ">{}\n{}\n", range, seq)?;
                }
            }
        }
    }

    Ok(())
}
