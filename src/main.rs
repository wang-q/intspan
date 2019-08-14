#[macro_use]
extern crate clap;

mod cli;
mod test;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("test") {
        test::run_test();
    }
}
