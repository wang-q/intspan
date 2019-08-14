#[macro_use]
extern crate clap;

mod cli;
mod test;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        test::run_test();
    }
}
