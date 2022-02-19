use clap::*;
use log::{debug, info, warn};
use simplelog::*;
use std::fs::File;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> App<'a> {
    App::new("txdb")
        .about("Init the taxonomy database")
        .after_help(
            "\
             ~/.nwr/taxonomy.sqlite\n\
             ",
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .takes_value(true)
                .help("Change working directory"),
        )
}

static DDL_TX: &str = r###"
DROP TABLE IF EXISTS division;
DROP TABLE IF EXISTS node;
DROP TABLE IF EXISTS name;

CREATE TABLE IF NOT EXISTS division (
    id       INTEGER      NOT NULL
                          PRIMARY KEY,
    division VARCHAR (50) NOT NULL
);

CREATE TABLE IF NOT EXISTS node (
    tax_id        INTEGER      NOT NULL
                               PRIMARY KEY,
    parent_tax_id INTEGER,
    rank          VARCHAR (25) NOT NULL,
    division_id   INTEGER      NOT NULL,
    comment       TEXT,
    FOREIGN KEY (
        division_id
    )
    REFERENCES division (id)
);

CREATE TABLE IF NOT EXISTS name (
    id         INTEGER      NOT NULL
                            PRIMARY KEY,
    tax_id     INTEGER      NOT NULL,
    name       VARCHAR (50) NOT NULL,
    name_class VARCHAR (50) NOT NULL
);
"###;

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());

    let nwrdir = if args.is_present("dir") {
        std::path::Path::new(args.value_of("dir").unwrap()).to_path_buf()
    } else {
        intspan::nwr_path()
    };
    let file = nwrdir.join("taxonomy.sqlite");
    if file.exists() {
        std::fs::remove_file(&file).unwrap();
    }

    info!("==> Opening database");
    let conn = rusqlite::Connection::open(file)?;
    conn.execute_batch(
        "
        PRAGMA journal_mode = OFF;
        PRAGMA synchronous = 0;
        PRAGMA cache_size = 1000000;
        PRAGMA locking_mode = EXCLUSIVE;
        PRAGMA temp_store = MEMORY;
        ",
    )?;

    info!("==> Create tables");
    conn.execute_batch(DDL_TX)?;

    // divisions
    info!("==> Loading division.dmp");
    {
        let dmp = File::open(nwrdir.join("division.dmp"))?;
        let mut csv = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_reader(dmp);

        let mut stmts: Vec<String> = vec![String::from("BEGIN;")];

        for result in csv.records() {
            let record = result?;
            let id: i64 = record[0].trim().parse()?;
            let name: String = record[2].trim().parse()?;

            stmts.push(format!(
                "INSERT INTO division VALUES ({}, '{}');",
                id,
                name.replace("'", "''")
            ));
        }

        stmts.push(String::from("COMMIT;"));
        let stmt = &stmts.join("\n");
        conn.execute_batch(stmt)?;
        debug!("Done inserting divisions");
    }

    // names
    info!("==> Loading names.dmp");
    {
        let dmp = File::open(nwrdir.join("names.dmp"))?;
        let mut csv = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_reader(dmp);

        let mut stmts: Vec<String> = vec![String::from("BEGIN;")];
        for (i, result) in csv.records().enumerate() {
            if i > 1 && i % 1000 == 0 {
                stmts.push(String::from("COMMIT;"));
                let stmt = &stmts.join("\n");
                conn.execute_batch(stmt)?;
                stmts.clear();
                stmts.push(String::from("BEGIN;"));
            }
            if i > 1 && i % 100000 == 0 {
                debug!("Read {} records", i);
            }

            let record = result?;

            // tax_id, name, unique_name, name_class
            let tax_id: i64 = record[0].trim().parse()?;
            let name: String = record[1].parse()?;
            let name_class: String = record[3].parse()?;

            stmts.push(format!(
                "
                INSERT INTO name(tax_id, name, name_class)
                VALUES ({}, '{}', '{}');
                ",
                tax_id.to_string(),
                name.trim().replace("'", "''"),
                name_class.trim().replace("'", "''")
            ));
        }

        // There could left records in stmts
        stmts.push(String::from("COMMIT;"));
        let stmt = &stmts.join("\n");
        conn.execute_batch(stmt)?;

        debug!("Creating indexes for name");
        conn.execute("CREATE INDEX idx_name_tax_id ON name(tax_id);", [])?;
        conn.execute("CREATE INDEX idx_name_name ON name(name);", [])?;
    }

    // nodes
    info!("==> Loading nodes.dmp");
    {
        let dmp = File::open(nwrdir.join("nodes.dmp"))?;
        let mut csv = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_reader(dmp);

        let mut stmts: Vec<String> = vec![String::from("BEGIN;")];

        for (i, result) in csv.records().enumerate() {
            if i > 1 && i % 1000 == 0 {
                stmts.push(String::from("COMMIT;"));
                let stmt = &stmts.join("\n");
                conn.execute_batch(stmt)?;
                stmts.clear();
                stmts.push(String::from("BEGIN;"));
            }
            if i > 1 && i % 100000 == 0 {
                debug!("Read {} records", i);
            }

            let record = result?;

            // tax_id, parent, rank, code, divid, undef, gen_code, undef, mito
            let tax_id: i64 = record[0].trim().parse()?;
            let parent_tax_id: i64 = record[1].trim().parse()?;
            let rank: String = record[2].trim().parse()?;
            let division_id: i64 = record[4].trim().parse()?;
            let comments: String = record[12].trim().parse()?;

            stmts.push(format!(
                "INSERT INTO node VALUES ({}, {}, '{}', {}, '{}');",
                tax_id.to_string(),
                parent_tax_id.to_string(),
                rank,
                division_id.to_string(),
                comments
            ));
        }

        // There could left records in stmts
        stmts.push(String::from("COMMIT;"));
        let stmt = &stmts.join("\n");
        conn.execute_batch(stmt)?;

        debug!("Creating indexes for node");
        conn.execute(
            "CREATE INDEX idx_node_parent_id ON node(parent_tax_id);",
            [],
        )?;
    }

    Ok(())
}
