extern crate clap;
use clap::*;

mod cmd_spanr;

fn main() -> std::io::Result<()> {
    let app = Command::new("spanr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`spanr` operates chromosome IntSpan files")
        .propagate_version(true)
        .arg_required_else_help(true)
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
        .subcommand(cmd_spanr::coverage::make_subcommand())
        .subcommand(cmd_spanr::gff::make_subcommand())
        .subcommand(cmd_spanr::convert::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("genome", sub_matches)) => cmd_spanr::genome::execute(sub_matches),
        Some(("some", sub_matches)) => cmd_spanr::some::execute(sub_matches),
        Some(("merge", sub_matches)) => cmd_spanr::merge::execute(sub_matches),
        Some(("split", sub_matches)) => cmd_spanr::split::execute(sub_matches),
        Some(("stat", sub_matches)) => cmd_spanr::stat::execute(sub_matches),
        Some(("statop", sub_matches)) => cmd_spanr::statop::execute(sub_matches),
        Some(("combine", sub_matches)) => cmd_spanr::combine::execute(sub_matches),
        Some(("compare", sub_matches)) => cmd_spanr::compare::execute(sub_matches),
        Some(("span", sub_matches)) => cmd_spanr::span::execute(sub_matches),
        Some(("cover", sub_matches)) => cmd_spanr::cover::execute(sub_matches),
        Some(("coverage", sub_matches)) => cmd_spanr::coverage::execute(sub_matches),
        Some(("gff", sub_matches)) => cmd_spanr::gff::execute(sub_matches),
        Some(("convert", sub_matches)) => cmd_spanr::convert::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}

// Variable naming conventions
// set, runlists: single name IntSpan set or runlists
//      set is a set of IntSpans
//      set: BTreeMap<String, IntSpan>
//      runlists: BTreeMap<String, String>
// s_of, r_of: multiple names IntSpan or runlist
//      name ==> chr ==> IntSpan
//      name ==> chr ==> String
// json: BTreeMap<String, Value>, single or multiple json
// res: result, single name IntSpan set
//      BTreeMap<String, IntSpan>
// res_of: BTreeMap<String, BTreeMap<String, IntSpan>>
// sizes: chr.sizes, BTreeMap<String, i32>
// iv_of: BTreeMap<String, Vec<Iv>>

// TODO: use json to replace yaml
