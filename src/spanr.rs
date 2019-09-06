extern crate clap;
use clap::*;

mod cmd_spanr;

fn main() {
    let app = App::new("intspan")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`intspan` operates chromosome IntSpan files")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_spanr::genome::make_subcommand())
        .subcommand(cmd_spanr::some::make_subcommand())
        .subcommand(cmd_spanr::merge::make_subcommand())
        .subcommand(cmd_spanr::split::make_subcommand())
        .subcommand(cmd_spanr::stat::make_subcommand())
        .subcommand(cmd_spanr::statop::make_subcommand())
        .subcommand(cmd_spanr::combine::make_subcommand())
        .subcommand(cmd_spanr::compare::make_subcommand())
        .subcommand(cmd_spanr::span::make_subcommand())
        .subcommand(cmd_spanr::cover::make_subcommand())
        .subcommand(cmd_spanr::gff::make_subcommand())
        .subcommand(cmd_spanr::convert::make_subcommand())
        .subcommand(cmd_spanr::range::make_subcommand());

    // Check which subcomamnd the user ran...
    let _res = match app.get_matches().subcommand() {
        ("genome", Some(sub_matches)) => cmd_spanr::genome::execute(sub_matches),
        ("some", Some(sub_matches)) => cmd_spanr::some::execute(sub_matches),
        ("merge", Some(sub_matches)) => cmd_spanr::merge::execute(sub_matches),
        ("split", Some(sub_matches)) => cmd_spanr::split::execute(sub_matches),
        ("stat", Some(sub_matches)) => cmd_spanr::stat::execute(sub_matches),
        ("statop", Some(sub_matches)) => cmd_spanr::statop::execute(sub_matches),
        ("combine", Some(sub_matches)) => cmd_spanr::combine::execute(sub_matches),
        ("compare", Some(sub_matches)) => cmd_spanr::compare::execute(sub_matches),
        ("span", Some(sub_matches)) => cmd_spanr::span::execute(sub_matches),
        ("cover", Some(sub_matches)) => cmd_spanr::cover::execute(sub_matches),
        ("gff", Some(sub_matches)) => cmd_spanr::gff::execute(sub_matches),
        ("convert", Some(sub_matches)) => cmd_spanr::convert::execute(sub_matches),
        ("range", Some(sub_matches)) => cmd_spanr::range::execute(sub_matches),
        (_, _) => unreachable!(),
    };
}

// TODO: CI releases
// TODO: wrap IO with Result
// TODO: satisfy clippy
// TODO: impl various traits

// set, runlists: single name IntSpan set or runlists
//      set is a set of IntSpans
//      set: BTreeMap<String, IntSpan>
//      runlists: BTreeMap<String, String>
// s_of, r_of: multiple names IntSpan or runlist
//      name ==> chr ==> IntSpan
//      name ==> chr ==> String
// yaml: BTreeMap<String, Value>, single or multiple yaml
// res: result, single name IntSpan set
//      BTreeMap<String, IntSpan>
// res_of: BTreeMap<String, BTreeMap<String, IntSpan>>
// sizes: chr.sizes, BTreeMap<String, i32>
