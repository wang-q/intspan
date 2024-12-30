use clap::*;
use cmd_lib::*;
use intspan::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{env, fs};
use tempfile::TempDir;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("pl-2rmp")
        .about("Pipeline - Two Rounds of Merging and Replacing")
        .after_help(
            r###"
* The computational complexity of merging is O(n^2), splitting it into two
  rounds will greatly reduce the computation time

* <infiles> are paths to .rg or .tsv files, .gz is supported
    * infile == stdin means reading from STDIN

* This pipeline depends on the binary `rgr` itself

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .index(1)
                .help("Input files to process. Multiple files can be specified"),
        )
        .arg(
            Arg::new("coverage")
                .long("coverage")
                .short('c')
                .num_args(1)
                .default_value("0.95")
                .value_parser(value_parser!(f32))
                .help("Ranges with coverage larger than this value will be merged"),
        )
        .arg(
            Arg::new("line_limit")
                .long("line")
                .num_args(1)
                .default_value("1000")
                .value_parser(value_parser!(usize))
                .help("Each split having this number of lines"),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .num_args(1)
                .default_value("stdout")
                .help("Output filename. [stdout] for screen"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let coverage = *args.get_one::<f32>("coverage").unwrap();
    let line_limit = *args.get_one::<usize>("line_limit").unwrap();
    let outfile = args.get_one::<String>("outfile").unwrap();

    let curdir = env::current_dir()?;
    let rgr = env::current_exe().unwrap().display().to_string();
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    run_cmd!(info "==> Paths")?;
    run_cmd!(info "    \"rgr\"   = ${rgr}")?;
    run_cmd!(info "    \"curdir\" = ${curdir}")?;
    run_cmd!(info "    \"tempdir\" = ${tempdir_str}")?;

    //----------------------------
    // Operating
    //----------------------------
    run_cmd!(info "==> Basenames and absolute paths")?;
    // basename => abs_path
    let mut abs_infiles = vec![];
    for infile in args.get_many::<String>("infiles").unwrap() {
        if infile == "stdin" {
            abs_infiles.push("stdin".to_string());
        } else {
            let absolute = intspan::absolute_path(infile)
                .unwrap()
                .display()
                .to_string();

            abs_infiles.push(absolute.to_string());
        }
    }

    run_cmd!(info "==> Switch to tempdir")?;
    env::set_current_dir(tempdir_str)?;

    run_cmd!(info "==> Splitting")?;
    let mut serial = 1;
    let mut out_ranges = vec![];
    // save inputs, so we can resue stdin latter
    let mut round1 = File::create("r1.lines")?;
    for infile in abs_infiles.iter() {
        let reader = reader(infile);
        for line in reader.lines().map_while(Result::ok) {
            round1.write_fmt(format_args!("{}\n", &line))?;

            for part in line.split('\t') {
                let range = Range::from_str(part);
                if !range.is_valid() {
                    continue;
                }
                out_ranges.push(part.to_string());

                if out_ranges.len() >= line_limit {
                    let outfile = format!("split.{}", serial);
                    write_lines(
                        outfile.as_str(),
                        &out_ranges.iter().map(AsRef::as_ref).collect(),
                    )?;

                    // clear caches
                    out_ranges = vec![];
                    // bump serial
                    serial += 1;
                }
            }
        }
    }
    round1.flush()?;

    // last part
    if !out_ranges.is_empty() {
        let outfile = format!("split.{}", serial);
        write_lines(
            outfile.as_str(),
            &out_ranges.iter().map(AsRef::as_ref).collect(),
        )?;
    } else {
        serial -= 1;
    }
    run_cmd!(info "    Total" ${serial} "splits")?;

    run_cmd!(info "==> 1st round of merging")?;
    for i in 1..=serial {
        let infile = format!("split.{}", i);
        if Path::new(infile.as_str()).is_file() {
            run_cmd!(info "   " ${infile})?;
            run_cmd!(
                rgr merge ${infile} -c ${coverage} -o replace.${i}
            )?;
            run_cmd!(
                rgr replace ${infile} replace.${i} -o replaced.${i}
            )?;
        }
    }

    run_cmd!(info "==> Results of 1st round")?;
    {
        let mut merged_1st: BTreeSet<String> = BTreeSet::new();
        for i in 1..=serial {
            let infile = format!("replaced.{}", i);
            if Path::new(infile.as_str()).is_file() {
                let reader = reader(&infile);
                for line in reader.lines().map_while(Result::ok) {
                    merged_1st.insert(line);
                }
            }
        }
        write_lines(
            "1st.replace",
            &merged_1st.iter().map(AsRef::as_ref).collect(),
        )?;
        let count_1st = merged_1st.len();
        run_cmd!(info "   " ${count_1st})?;

        let mut writer_1st_replace = writer("1st.replace.tsv");
        for i in 1..=serial {
            let infile = format!("replace.{}", i);
            if Path::new(infile.as_str()).is_file() {
                let reader = reader(&infile);
                for line in reader.lines().map_while(Result::ok) {
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() == 2 {
                        writer_1st_replace.write_all((line + "\n").as_ref())?;
                    }
                }
            }
        }
    }

    run_cmd!(info "==> 2nd round of merging")?;
    run_cmd!(
        rgr merge 1st.replace -c ${coverage} -o 2nd.replace.tsv
    )?;

    run_cmd!(info "==> 1st round of replacing")?;
    run_cmd!(
        cat r1.lines |
            rgr replace stdin 1st.replace.tsv -o replaced.1st
    )?;

    run_cmd!(info "==> 2nd round of replacing")?;
    run_cmd!(
        rgr replace replaced.1st 2nd.replace.tsv -o replaced.2st
    )?;

    run_cmd!(info "==> Remove temp files")?;
    for i in 1..=serial {
        let infile = format!("split.{}", i);
        if Path::new(infile.as_str()).is_file() {
            fs::remove_file(infile.as_str())?;
        }

        let infile = format!("replace.{}", i);
        if Path::new(infile.as_str()).is_file() {
            fs::remove_file(infile.as_str())?;
        }

        let infile = format!("replaced.{}", i);
        if Path::new(infile.as_str()).is_file() {
            fs::remove_file(infile.as_str())?;
        }
    }

    //----------------------------
    // Done
    //----------------------------
    if outfile == "stdout" {
        run_cmd!(cat replaced.2st)?;
    } else {
        env::set_current_dir(&curdir)?;
        fs::copy(
            tempdir.path().join("replaced.2st").to_str().unwrap(),
            outfile,
        )?;
    }

    Ok(())
}

// fn pause() {
//     let mut stdin = io::stdin();
//     let mut stdout = io::stdout();
//
//     // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
//     write!(stdout, "Press any key to continue...").unwrap();
//     stdout.flush().unwrap();
//
//     // Read a single byte and discard
//     let _ = stdin.read(&mut [0u8]).unwrap();
// }
