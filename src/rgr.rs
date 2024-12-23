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
        .color(ColorChoice::Auto)
        .subcommand(cmd_rgr::count::make_subcommand())
        .subcommand(cmd_rgr::dedup::make_subcommand())
        .subcommand(cmd_rgr::field::make_subcommand())
        .subcommand(cmd_rgr::filter::make_subcommand())
        .subcommand(cmd_rgr::keep::make_subcommand())
        .subcommand(cmd_rgr::md::make_subcommand())
        .subcommand(cmd_rgr::merge::make_subcommand())
        .subcommand(cmd_rgr::pl_2rmp::make_subcommand())
        .subcommand(cmd_rgr::prop::make_subcommand())
        .subcommand(cmd_rgr::replace::make_subcommand())
        .subcommand(cmd_rgr::runlist::make_subcommand())
        .subcommand(cmd_rgr::select::make_subcommand())
        .subcommand(cmd_rgr::sort::make_subcommand())
        .subcommand(cmd_rgr::span::make_subcommand())
        .after_help(
            r###"
File formats

* .rg files are single-column .tsv
* Field numbers in the TSV file start at 1

Subcommand groups:

* Generic .tsv
    * dedup / keep / md / replace / filter / select
* Single range field
    * field / sort / count / prop / span / runlist
* Multiple range fields
    * merge / pl-2rmp

"###,
        );

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        // Generic .tsv
        Some(("dedup", sub_matches)) => cmd_rgr::dedup::execute(sub_matches),
        Some(("keep", sub_matches)) => cmd_rgr::keep::execute(sub_matches),
        Some(("md", sub_matches)) => cmd_rgr::md::execute(sub_matches),
        Some(("replace", sub_matches)) => cmd_rgr::replace::execute(sub_matches),
        Some(("filter", sub_matches)) => cmd_rgr::filter::execute(sub_matches),
        Some(("select", sub_matches)) => cmd_rgr::select::execute(sub_matches),
        // Single range field
        Some(("field", sub_matches)) => cmd_rgr::field::execute(sub_matches),
        Some(("sort", sub_matches)) => cmd_rgr::sort::execute(sub_matches),
        Some(("count", sub_matches)) => cmd_rgr::count::execute(sub_matches),
        Some(("prop", sub_matches)) => cmd_rgr::prop::execute(sub_matches),
        Some(("span", sub_matches)) => cmd_rgr::span::execute(sub_matches),
        Some(("runlist", sub_matches)) => cmd_rgr::runlist::execute(sub_matches),
        // Multiple range fields
        Some(("merge", sub_matches)) => cmd_rgr::merge::execute(sub_matches),
        Some(("pl-2rmp", sub_matches)) => cmd_rgr::pl_2rmp::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}

// TODO: --bed for `rgr field`
