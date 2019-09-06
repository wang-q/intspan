extern crate clap;
use clap::*;

mod cmd_ovlpr;

fn main() {
    let app = App::new("ovlpr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`ovlpr` operates overlaps between sequences")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_ovlpr::covered::make_subcommand());

    // Check which subcomamnd the user ran...
    let _res = match app.get_matches().subcommand() {
        ("covered", Some(sub_matches)) => cmd_ovlpr::covered::execute(sub_matches),
        (_, _) => unreachable!(),
    };
}
