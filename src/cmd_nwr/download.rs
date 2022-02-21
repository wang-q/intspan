use clap::*;
use log::{info, warn};
use simplelog::*;
use std::fs::File;
use std::io;

// Create clap subcommand arguments
pub fn make_subcommand<'a>() -> Command<'a> {
    Command::new("download")
        .about("Download the latest release of `taxdmp`")
        .after_help(
            "\
             curl -LO https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdmp.zip\n\
             curl -LO https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdmp.zip.md5\n\
             mv taxdump.* ~/.nwr/\n\
             ",
        )
        .arg(
            Arg::new("host")
                .long("host")
                .takes_value(true)
                .default_value("ftp.ncbi.nih.gov:21")
                .forbid_empty_values(true)
                .help("NCBI FTP Host:Port"),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .takes_value(true)
                .default_value("/pub/taxonomy")
                .forbid_empty_values(true)
                .help("NCBI FTP Path"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let nwrdir = intspan::nwr_path();
    let path = nwrdir.join("taxdmp.zip");

    // download
    info!(
        "==> Downloading from {} ...",
        args.value_of("host").unwrap()
    );
    if std::path::Path::new(&path).exists() {
        info!("Skipping, dump file exists");
    } else {
        info!("Connecting...");
        let mut conn = ftp::FtpStream::connect(args.value_of("host").unwrap())?;
        conn.login("ftp", "example@example.com")?;
        info!("Connected.");
        conn.cwd(args.value_of("path").unwrap())?;
        info!("Remote directory: {}", conn.pwd().unwrap());

        info!("Retrieving MD5 file...");
        let mut file = File::create(nwrdir.join("taxdmp.zip.md5"))?;
        let mut cursor = conn.simple_retr("taxdmp.zip.md5")?;
        io::copy(&mut cursor, &mut file)?;

        info!("Retrieving dump file...");
        conn.retr("taxdmp.zip", |stream| {
            let mut file = match File::create(&path) {
                Err(e) => return Err(ftp::FtpError::ConnectionError(e)),
                Ok(f) => f,
            };
            io::copy(stream, &mut file).map_err(ftp::FtpError::ConnectionError)
        })?;

        conn.quit()?;
        info!("End connection.");
    }

    // check
    info!("==> Checking...");
    {
        let mut file = File::open(&path)?;
        let mut hasher = md5::Context::new();
        info!("Computing MD5 sum...");
        io::copy(&mut file, &mut hasher)?;
        let digest = format!("{:x}", hasher.compute());

        let mut ncbi_digest = std::fs::read_to_string(nwrdir.join("taxdmp.zip.md5"))?;
        ncbi_digest.truncate(32);

        if digest != ncbi_digest {
            warn!("Expected sum is: {}", ncbi_digest);
            warn!("Computed sum is: {}", digest);
            panic!("Fail to check integrity.");
        } else {
            info!("MD5 sum passed");
        }
    }

    // extract
    info!("==> Extracting...");
    {
        let file = File::open(&path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = nwrdir.join(file.mangled_name());

            info!("Extracted {}", outpath.as_path().display());
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
