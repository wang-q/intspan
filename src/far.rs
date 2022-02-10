extern crate clap;
use clap::*;

mod cmd_far;

fn main() -> std::io::Result<()> {
    let app = App::new("far")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`far` is a lightweight tool for operating sequences in the fasta format")
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_far::size::make_subcommand())
        .subcommand(cmd_far::some::make_subcommand())
        .subcommand(cmd_far::region::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("size", sub_matches)) => cmd_far::size::execute(sub_matches),
        Some(("some", sub_matches)) => cmd_far::some::execute(sub_matches),
        Some(("region", sub_matches)) => cmd_far::region::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}
