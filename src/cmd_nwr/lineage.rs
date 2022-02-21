use clap::*;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("lineage")
        .about("Output the lineage of the term")
        .arg(
            Arg::new("term")
                .help("The NCBI Taxonomy ID or scientific name")
                .required(true)
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

    let term = args.value_of("term").unwrap().to_string();
    let id = intspan::term_to_tax_id(&conn, term).unwrap();

    let lineage = intspan::get_lineage(&conn, id).unwrap();

    for node in lineage.iter() {
        writer.write_fmt(format_args!(
            "{}\t{}\t{}\n",
            node.rank,
            node.names.get("scientific name").unwrap()[0],
            node.tax_id
        ))?;
    }

    Ok(())
}
