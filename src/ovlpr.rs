extern crate clap;
use clap::*;

mod cmd_ovlpr;

fn main() -> std::io::Result<()> {
    let app = App::new("ovlpr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`ovlpr` operates overlaps between sequences")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_ovlpr::covered::make_subcommand())
        .subcommand(cmd_ovlpr::paf2ovlp::make_subcommand())
        .subcommand(cmd_ovlpr::replace::make_subcommand())
        .subcommand(cmd_ovlpr::restrict::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        ("covered", Some(sub_matches)) => cmd_ovlpr::covered::execute(sub_matches),
        ("paf2ovlp", Some(sub_matches)) => cmd_ovlpr::paf2ovlp::execute(sub_matches),
        ("replace", Some(sub_matches)) => cmd_ovlpr::replace::execute(sub_matches),
        ("restrict", Some(sub_matches)) => cmd_ovlpr::restrict::execute(sub_matches),
        (_, _) => unreachable!(),
    }?;

    Ok(())
}
