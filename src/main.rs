#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

mod cmd;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        //        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::test::make_subcommand());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        //        ("init", Some(sub_matches)) => cmd::init::execute(sub_matches),
        ("test", Some(sub_matches)) => cmd::test::execute(sub_matches),
        (_, _) => unreachable!(),
    };

}
