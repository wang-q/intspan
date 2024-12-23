use clap::*;
use std::io::{BufRead, Write};

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("keep")
        .about("Keep the the initial header line")
        .after_help(
            r###"
The first line of each file is treated as a header and the one of first file is output unchanged.
Subsequent lines are sent to the specified command via standard input, excluding headers of other files.
The output from the command is appended to the initial header.

Use a double hyphen (--) to separate the command from the file arguments.

"###,
        )
        .arg(
            Arg::new("infiles")
                .required(true)
                .num_args(1..)
                .help("Sets the input file(s) to use"),
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
    let mut header_written = false;

    for infile in infiles {
        let reader = intspan::reader(infile);

        let mut lines = reader.lines();
        if let Some(first_line) = lines.next() {
            let first_line = first_line?;
            if !header_written {
                println!("{}", first_line);
                header_written = true;
            }

            for line in lines {
                let line = line?;
                if let Some(ref mut stdin) = child.stdin {
                    writeln!(stdin, "{}", line)?;
                }
            }
        }
    }
    if let Some(ref mut stdin) = child.stdin {
        stdin.flush()?;
    }

    let _ = child.wait()?;
    Ok(())
}
