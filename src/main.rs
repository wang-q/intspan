extern crate clap;

use clap::*;

mod cmd;
mod utils;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(cmd::test::make_subcommand())
        .subcommand(cmd::genome::make_subcommand())
        .subcommand(cmd::some::make_subcommand())
        .subcommand(cmd::merge::make_subcommand())
        .subcommand(cmd::split::make_subcommand())
        .subcommand(cmd::stat::make_subcommand())
        .subcommand(cmd::statop::make_subcommand())
        .subcommand(cmd::combine::make_subcommand())
        .subcommand(cmd::compare::make_subcommand())
        .subcommand(cmd::span::make_subcommand());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        ("test", Some(sub_matches)) => cmd::test::execute(sub_matches),
        ("genome", Some(sub_matches)) => cmd::genome::execute(sub_matches),
        ("some", Some(sub_matches)) => cmd::some::execute(sub_matches),
        ("merge", Some(sub_matches)) => cmd::merge::execute(sub_matches),
        ("split", Some(sub_matches)) => cmd::split::execute(sub_matches),
        ("stat", Some(sub_matches)) => cmd::stat::execute(sub_matches),
        ("statop", Some(sub_matches)) => cmd::statop::execute(sub_matches),
        ("combine", Some(sub_matches)) => cmd::combine::execute(sub_matches),
        ("compare", Some(sub_matches)) => cmd::compare::execute(sub_matches),
        ("span", Some(sub_matches)) => cmd::span::execute(sub_matches),
        (_, _) => unreachable!(),
    };
}
