#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

mod cmd;
mod utils;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(cmd::test::make_subcommand())
        .subcommand(cmd::genome::make_subcommand())
        .subcommand(cmd::some::make_subcommand());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        ("test", Some(sub_matches)) => cmd::test::execute(sub_matches),
        ("genome", Some(sub_matches)) => cmd::genome::execute(sub_matches),
        ("some", Some(sub_matches)) => cmd::some::execute(sub_matches),
        (_, _) => unreachable!(),
    };
}
