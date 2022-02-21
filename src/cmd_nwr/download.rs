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
            r###"
You can download the tarball manually.

curl -LO https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump.tar.gz
curl -LO https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump.tar.gz.md5
mv taxdump.* ~/.nwr/
"###,
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
    let tarball = nwrdir.join("taxdump.tar.gz");

    // download
    info!(
        "==> Downloading from {} ...",
        args.value_of("host").unwrap()
    );
    if std::path::Path::new(&tarball).exists() {
        info!("Skipping, dump file exists");
    } else {
        info!("Connecting...");
        let mut conn = ftp::FtpStream::connect(args.value_of("host").unwrap())?;
        conn.login("ftp", "example@example.com")?;
        info!("Connected.");
        conn.cwd(args.value_of("path").unwrap())?;
        info!("Remote directory: {}", conn.pwd().unwrap());

        info!("Retrieving MD5 file...");
        let mut file = File::create(nwrdir.join("taxdump.tar.gz.md5"))?;
        let mut cursor = conn.simple_retr("taxdump.tar.gz.md5")?;
        io::copy(&mut cursor, &mut file)?;

        info!("Retrieving dump file...");
        conn.retr("taxdump.tar.gz", |stream| {
            let mut file = match File::create(&tarball) {
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
        let mut file = File::open(&tarball)?;
        let mut hasher = md5::Context::new();
        info!("Computing MD5 sum...");
        io::copy(&mut file, &mut hasher)?;
        let digest = format!("{:x}", hasher.compute());

        let mut ncbi_digest = std::fs::read_to_string(nwrdir.join("taxdump.tar.gz.md5"))?;
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
        let tar_gz = File::open(&tarball)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);

        let mut archive = tar::Archive::new(tar);
        archive.unpack(nwrdir)?;
    }

    Ok(())
}
