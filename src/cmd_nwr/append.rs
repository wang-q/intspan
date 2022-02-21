use clap::*;
use intspan::Node;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> App<'a> {
    App::new("append")
        .about("Append fields of higher ranks to a TSV file")
        .after_help(
            r###"
* If `--rank` is empty, the scientific name will be appended.

* Valid ranks
  * species genus family order class phylum kingdom
  * Use other ranks, such as clade or no rank, at your own risk.

* If the desired rank does not present, `NA` will be appended.

* Lines starting with "#" will be treated as headers and have ranks attached to them.

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .min_values(1)
                .index(1)
                .help("Input filename. [stdin] for standard input"),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .takes_value(true)
                .help("Change the database directory"),
        )
        .arg(
            Arg::new("rank")
                .long("rank")
                .short('r')
                .takes_value(true)
                .multiple_occurrences(true)
                .help("To append which rank(s)"),
        )
        .arg(
            Arg::new("column")
                .long("column")
                .short('c')
                .takes_value(true)
                .default_value("1")
                .forbid_empty_values(true)
                .help("The column where the IDs are located, starting from 1"),
        )
        .arg(Arg::new("id").long("id").help("Also append rank id"))
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
    let mut writer = intspan::writer(args.value_of("outfile").unwrap());

    let column: usize = args.value_of_t("column").unwrap_or_else(|e| {
        eprintln!("Need a integer for --column\n{}", e);
        std::process::exit(1)
    });

    let nwrdir = if args.is_present("dir") {
        std::path::Path::new(args.value_of("dir").unwrap()).to_path_buf()
    } else {
        intspan::nwr_path()
    };

    let conn = intspan::connect_txdb(&nwrdir).unwrap();

    let mut ranks = vec![];
    if args.is_present("rank") {
        for rank in args.values_of("rank").unwrap() {
            ranks.push(rank.to_string());
        }
    }
    let is_id = args.is_present("id");

    for infile in args.values_of("infiles").unwrap() {
        let reader = intspan::reader(infile);

        for line in reader.lines().filter_map(|r| r.ok()) {
            let mut fields: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();
            let new_line: String;

            // Lines start with "#"
            if line.starts_with("#") {
                if ranks.len() == 0 {
                    fields.push("sci_name".to_string());
                    if is_id {
                        fields.push("tax_id".to_string());
                    }
                } else {
                    for rank in ranks.iter() {
                        fields.push(rank.to_string());
                        if is_id {
                            fields.push(format!("{} id", rank));
                        }
                    }
                }

                new_line = fields.join("\t");
            }
            // Normal lines
            else {
                // Check the given field
                let term = fields.get(column - 1).unwrap();
                let id = intspan::term_to_tax_id(&conn, term.to_string()).unwrap();

                if ranks.len() == 0 {
                    let node = intspan::get_node(&conn, vec![id])
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .clone();
                    let s = &node.names.get("scientific name").unwrap()[0];

                    fields.push(s.to_string());
                    if is_id {
                        fields.push(format!("{}", id));
                    }
                } else {
                    let lineage = intspan::get_lineage(&conn, id).unwrap();
                    for rank in ranks.iter() {
                        let (tax_id, sci_name) = find_rank(&lineage, rank.to_string());
                        fields.push(sci_name.to_string());
                        if is_id {
                            fields.push(format!("{}", tax_id));
                        }
                    }
                }

                new_line = fields.join("\t");
            }

            writer.write_fmt(format_args!("{}\n", new_line))?;
        }
    }

    Ok(())
}

fn find_rank(lineage: &Vec<Node>, rank: String) -> (i64, String) {
    let mut tax_id: i64 = 0;
    let mut sci_name = "NA".to_string();

    for node in lineage.into_iter() {
        if node.rank == rank {
            sci_name = (&node.names.get("scientific name").unwrap()[0]).to_string();
            tax_id = node.tax_id;
            break;
        }
    }

    (tax_id, sci_name)
}
