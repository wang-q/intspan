use clap::*;
use intspan::*;
use itertools::Itertools;

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
            Arg::new("best")
                .long("best")
                .action(ArgAction::SetTrue)
                .help("best-to-best bilateral links"),
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
    let is_best = args.get_flag("best");

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
                    for j in i + 1..headers.len() {
                        writer.write_all(format!("{}\t{}\n", headers[i], headers[j]).as_ref())?;
                    }
                }
            } else if is_best {
                let mut best_pair: Vec<(usize, usize)> = vec![];
                for i in 0..headers.len() {
                    let mut dist_idx: (f32, usize) = (1.0, headers.len() - 1);
                    for j in 0..headers.len() {
                        if i == j {
                            continue;
                        }
                        let dist = pair_d(block.entries[i].seq(), block.entries[j].seq());
                        if dist < dist_idx.0 {
                            dist_idx = (dist, j);
                        }
                    }
                    if i < dist_idx.1 {
                        best_pair.push((i, dist_idx.1));
                    } else {
                        best_pair.push((dist_idx.1, i));
                    }
                }
                // from itertools
                let best_pair: Vec<(usize, usize)> = best_pair.into_iter().unique().collect();

                for (i, j) in best_pair {
                    writer.write_all(format!("{}\t{}\n", headers[i], headers[j]).as_ref())?;
                }
            } else {
                writer.write_all(format!("{}\n", headers.join("\t")).as_ref())?;
            }
        }
    }

    Ok(())
}
