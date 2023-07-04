extern crate clap;
use clap::*;

mod cmd_ovlpr;

fn main() -> anyhow::Result<()> {
    let app = Command::new("ovlpr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`ovlpr` operates overlaps between sequences")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(ColorChoice::Auto)
        .subcommand(cmd_ovlpr::covered::make_subcommand())
        .subcommand(cmd_ovlpr::paf2ovlp::make_subcommand())
        .subcommand(cmd_ovlpr::restrict::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("covered", sub_matches)) => cmd_ovlpr::covered::execute(sub_matches),
        Some(("paf2ovlp", sub_matches)) => cmd_ovlpr::paf2ovlp::execute(sub_matches),
        Some(("restrict", sub_matches)) => cmd_ovlpr::restrict::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}
