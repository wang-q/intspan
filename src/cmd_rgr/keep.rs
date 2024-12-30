use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("keep")
        .about("Keep the the initial header line")
        .after_help(
            r###"
The first N lines of each file is treated as a header and the one of first file is output unchanged.
Subsequent lines are sent to the specified command via stdin, excluding headers of other files.
The output from the command is appended to the initial header.

* Use a double hyphen (--) to separate the command from the file arguments.

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .help("Sets the input file(s) to use"),
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
                .help("Don't write headers"),
        )
        .arg(
            Arg::new("commands")
                .required(true)
                .num_args(1..)
                .last(true)
                .value_parser(value_parser!(String)),
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

    let mut first_file = true; // Track if we are processing the first file
    for infile in infiles {
        let reader = intspan::reader(infile);
        let mut header_written = 0;
        let mut lines = reader.lines();

        while let Some(line) = lines.next() {
            let line = line?;
            if header_written < opt_lines {
                if first_file && !is_delete {
                    // Only print headers from the first file
                    println!("{}", line);
                }
                header_written += 1;
            } else {
                // Send subsequent lines to the command
                if let Some(ref mut stdin) = child.stdin {
                    writeln!(stdin, "{}", line)?;
                }
            }
        }

        // After processing the first file, set first_file to false
        first_file = false;
    }

    if let Some(ref mut stdin) = child.stdin {
        stdin.flush()?;
    }

    let _ = child.wait()?;
    Ok(())
}
