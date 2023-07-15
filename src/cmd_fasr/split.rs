use clap::*;
use intspan::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("split")
        .about("Split block fasta files to per-alignment/chromosome fasta files")
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
            Arg::new("suffix")
                .long("suffix")
                .short('s')
                .num_args(1)
                .default_value(".fas")
                .help("Extensions of output files"),
        )
        .arg(
            Arg::new("chr")
                .long("chr")
                .action(ArgAction::SetTrue)
                .help("Split by chromosomes"),
        )
        .arg(
            Arg::new("simple")
                .long("simple")
                .action(ArgAction::SetTrue)
                .help("Only keep names in headers"),
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
    let is_chr = args.get_flag("chr");
    let is_simple = args.get_flag("simple");

    let mut file_of: BTreeMap<String, File> = BTreeMap::new();

    //----------------------------
    // Operating
    //----------------------------
    for infile in args.get_many::<String>("infiles").unwrap() {
        let mut reader = reader(infile);

        while let Ok(block) = next_fas_block(&mut reader) {
            let filename = if is_chr {
                let tname = block.entries.first().unwrap().range().name();
                let tchr = block.entries.first().unwrap().range().chr();
                format!("{}.{}", tname, tchr)
            } else {
                let trange = &block.entries.first().unwrap().range().clone();
                trange.to_string()
            }
            .replace(['(', ')', ':'], "_")
            .replace("__", "_");

            for entry in &block.entries {
                let range = entry.range().clone();
                let seq = std::str::from_utf8(entry.seq()).unwrap();

                //----------------------------
                // Output
                //----------------------------
                if outdir == "stdout" {
                    let header = if is_simple {
                        range.name().to_string()
                    } else {
                        range.to_string()
                    };
                    print!(">{}\n{}\n", header, seq);
                } else {
                    if !file_of.contains_key(&filename) {
                        let path = Path::new(outdir).join(filename.clone() + suffix);
                        let file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(path)?;
                        file_of.insert(filename.clone(), file);
                    }
                    write!(file_of.get(&filename).unwrap(), ">{}\n{}\n", range, seq)?;
                }
            }

            // end of a block
            if outdir == "stdout" {
                println!();
            } else {
                writeln!(file_of.get(&filename).unwrap())?;
            }
        }
    }

    Ok(())
}
