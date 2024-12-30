use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("keep")
        .about("Keep the the initial header line(s)")
        .after_help(
            r###"
The first N lines of each file is treated as a header and the one of first file is output unchanged.
Subsequent lines are sent to the specified command via stdin, excluding headers from other files.
The output from the command is appended to the initial header.

* Use a double hyphen (--) to separate the command from the file arguments.

Examples:
    # Keeps the first 2 lines of file1.txt as headers, processes the rest with `wc -l`
    rgr keep -l 2 file1.txt file2.txt -- wc -l

    # Skips headers and processes all lines with `sort`
    rgr keep --delete file1.txt file2.txt -- sort

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .help("Input file(s) to process"),
        )
        .arg(
            Arg::new("lines")
                .long("lines")
                .short('l')
                .num_args(1)
                .default_value("1")
                .value_parser(value_parser!(usize))
                .help("Number of header lines to keep"),
        )
        .arg(
            Arg::new("delete")
                .long("delete")
                .short('d')
                .action(ArgAction::SetTrue)
                .help("Skip writing headers"),
        )
        .arg(
            Arg::new("commands")
                .required(true)
                .num_args(1..)
                .last(true)
                .value_parser(value_parser!(String))
                .help("Command to process subsequent lines"),
        )
}

// command implementation
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let infiles = args
        .get_many::<String>("infiles")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    let opt_lines = *args.get_one::<usize>("lines").unwrap();
    let is_delete = args.get_flag("delete");

    let commands = args
        .get_many::<String>("commands")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    //----------------------------
    // Ops
    //----------------------------
    let mut child = std::process::Command::new(commands[0])
        .args(&commands[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;
    let stdin = child.stdin.as_mut().expect("Failed to open child stdin");

    let mut first_file = true; // Track if we are processing the first file
    for infile in infiles {
        let reader = intspan::reader(infile);
        let mut header_written = 0;
        let mut lines = reader.lines();

        while let Some(Ok(line)) = lines.next() {
            if header_written < opt_lines {
                if first_file && !is_delete {
                    // Only print headers from the first file
                    println!("{}", line);
                }
                header_written += 1;
            } else {
                // Send subsequent lines to the command
                writeln!(stdin, "{}", line)?;
            }
        }

        // After processing the first file, set first_file to false
        first_file = false;
    }

    stdin.flush()?;
    child.wait()?;
    Ok(())
}
