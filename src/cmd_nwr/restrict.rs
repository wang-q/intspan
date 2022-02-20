use clap::*;
use intspan::IntSpan;
use std::io::BufRead;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> App<'a> {
    App::new("restrict")
        .about("Restrict taxonomy terms to ancestral descendants")
        .after_help(
            "\
* All terms, including ancestors and fields in input files,
  are in the form of a Taxonomy ID or scientific name.

* Input files should be TSV.
  * `tests/nwr/taxon.tsv` as an example.

* Lines start with `#` will always be outputted.\
            ",
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
            Arg::new("file")
                .long("file")
                .short('f')
                .takes_value(true)
                .multiple_occurrences(true)
                .default_value("stdin")
                .forbid_empty_values(true)
                .help("Input filename. [stdin] for standard input"),
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

    let mut id_set = IntSpan::new();
    for term in args.values_of("terms").unwrap() {
        let id = intspan::term_to_tax_id(&conn, term.to_string()).unwrap();
        let descendents: Vec<i32> = intspan::get_all_descendent(&conn, id)
            .unwrap()
            .iter()
            .map(|n| *n as i32)
            .collect();
        id_set.add_vec(descendents.as_ref());
    }

    for infile in args.values_of("file").unwrap() {
        let reader = intspan::reader(infile);
        for line in reader.lines().filter_map(|r| r.ok()) {
            // Always output lines start with "#"
            if line.starts_with("#") {
                writer.write_fmt(format_args!("{}\n", line))?;
                continue;
            }

            // Check the given field
            let fields: Vec<&str> = line.split('\t').collect();
            let term = fields.get(column - 1).unwrap();
            let id = intspan::term_to_tax_id(&conn, term.to_string()).unwrap();

            if id_set.contains(id as i32) {
                writer.write_fmt(format_args!("{}\n", line))?;
            }
        }
    }

    Ok(())
}
