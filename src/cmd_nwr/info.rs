use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("info")
        .about("Information of Taxonomy ID(s) or scientific name(s)")
        .arg(
            Arg::new("terms")
                .help("The NCBI Taxonomy ID(s) or scientific name(s)")
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
            Arg::new("tsv")
                .long("tsv")
                .takes_value(false)
                .help("Output the results as TSV"),
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
    let mut writer = intspan::writer(args.value_of("outfile").unwrap());

    let nwrdir = if args.is_present("dir") {
        std::path::Path::new(args.value_of("dir").unwrap()).to_path_buf()
    } else {
        intspan::nwr_path()
    };

    let conn = intspan::connect_txdb(&nwrdir).unwrap();

    let mut ids = vec![];
    for term in args.values_of("terms").unwrap() {
        let id = intspan::term_to_tax_id(&conn, term.to_string()).unwrap();
        ids.push(id);
    }

    let nodes = intspan::get_node(&conn, ids).unwrap();

    if args.is_present("tsv") {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_writer(writer);

        wtr.write_record(&["#tax_id", "sci_name", "rank", "division"])?;
        for node in nodes.iter() {
            wtr.serialize((
                node.tax_id,
                &node.names.get("scientific name").unwrap()[0],
                &node.rank,
                &node.division,
            ))?;
        }
        wtr.flush()?;
    } else {
        for node in nodes.iter() {
            writer.write_fmt(format_args!("{}", node))?;
        }
    }

    Ok(())
}
