extern crate clap;
use clap::*;

mod cmd_rgr;

fn main() -> anyhow::Result<()> {
    let app = Command::new("rgr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`rgr` operates ranges in .rg and .tsv files")
        .propagate_version(true)
        .arg_required_else_help(true)
        .after_help(
            r###"
* Field numbers start with 1

"###,
        )
        .subcommand(cmd_rgr::count::make_subcommand())
        .subcommand(cmd_rgr::field::make_subcommand())
        .subcommand(cmd_rgr::merge::make_subcommand())
        .subcommand(cmd_rgr::prop::make_subcommand())
        .subcommand(cmd_rgr::replace::make_subcommand())
        .subcommand(cmd_rgr::runlist::make_subcommand())
        .subcommand(cmd_rgr::sort::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("count", sub_matches)) => cmd_rgr::count::execute(sub_matches),
        Some(("field", sub_matches)) => cmd_rgr::field::execute(sub_matches),
        Some(("merge", sub_matches)) => cmd_rgr::merge::execute(sub_matches),
        Some(("prop", sub_matches)) => cmd_rgr::prop::execute(sub_matches),
        Some(("replace", sub_matches)) => cmd_rgr::replace::execute(sub_matches),
        Some(("runlist", sub_matches)) => cmd_rgr::runlist::execute(sub_matches),
        Some(("sort", sub_matches)) => cmd_rgr::sort::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}

// TODO: `rgr span` 5p and 3p
// TODO: --bed for `rgr field`
