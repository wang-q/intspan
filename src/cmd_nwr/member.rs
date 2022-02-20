use clap::*;
use std::collections::HashSet;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> App<'a> {
    App::new("member")
        .about("List members (of certain ranks) under ancestral term(s)")
        .after_help(
            r###"
* Ancestral terms are in the form of a Taxonomy ID or scientific name.

* Valid ranks
  * species genus family order class phylum kingdom
  * Use other ranks, such as clade or no rank, at your own risk.

* The output file is in the same TSV format as `nwr info --tsv`.
"###,
        )
        .arg(
            Arg::new("terms")
                .help("The ancestor(s)")
                .required(true)
                .min_values(1)
                .index(1),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .takes_value(true)
                .help("Change working directory"),
        )
        .arg(
            Arg::new("rank")
                .long("rank")
                .short('r')
                .takes_value(true)
                .multiple_occurrences(true)
                .help("Which rank to list"),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .default_value("stdout")
                .forbid_empty_values(true)
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let writer = intspan::writer(args.value_of("outfile").unwrap());

    let nwrdir = if args.is_present("dir") {
        std::path::Path::new(args.value_of("dir").unwrap()).to_path_buf()
    } else {
        intspan::nwr_path()
    };

    let conn = intspan::connect_txdb(&nwrdir).unwrap();

    let mut tsv_wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(writer);
    tsv_wtr.write_record(&["#tax_id", "scientific_name", "rank", "division"])?;

    let mut rank_set: HashSet<String> = HashSet::new();
    if args.is_present("rank") {
        for rank in args.values_of("rank").unwrap() {
            rank_set.insert(rank.to_string());
        }
    }

    for term in args.values_of("terms").unwrap() {
        let id = intspan::term_to_tax_id(&conn, term.to_string()).unwrap();
        let descendents = intspan::get_all_descendent(&conn, id).unwrap();

        let nodes = intspan::get_node(&conn, descendents)?;

        for node in nodes.iter() {
            if !rank_set.is_empty() && !rank_set.contains(&node.rank) {
                continue;
            }

            tsv_wtr.serialize((
                node.tax_id,
                &node.names.get("scientific name").unwrap()[0],
                &node.rank,
                &node.division,
            ))?;
        }
    }
    tsv_wtr.flush()?;

    Ok(())
}
