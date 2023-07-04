extern crate clap;
use clap::*;

mod cmd_linkr;

fn main() -> anyhow::Result<()> {
    let app = Command::new("linkr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`linkr` operates ranges on chromosomes and links of ranges")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd_linkr::circos::make_subcommand())
        .subcommand(cmd_linkr::sort::make_subcommand())
        .subcommand(cmd_linkr::filter::make_subcommand())
        .subcommand(cmd_linkr::clean::make_subcommand())
        .subcommand(cmd_linkr::connect::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("circos", sub_matches)) => cmd_linkr::circos::execute(sub_matches),
        Some(("sort", sub_matches)) => cmd_linkr::sort::execute(sub_matches),
        Some(("filter", sub_matches)) => cmd_linkr::filter::execute(sub_matches),
        Some(("clean", sub_matches)) => cmd_linkr::clean::execute(sub_matches),
        Some(("connect", sub_matches)) => cmd_linkr::connect::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}
