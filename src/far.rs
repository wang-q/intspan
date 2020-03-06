extern crate clap;
use clap::*;

mod cmd_far;

fn main() -> std::io::Result<()> {
    let app = App::new("far")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`far` is a lightweight tool for operating sequences in the fasta format")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_far::region::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        ("region", Some(sub_matches)) => cmd_far::region::execute(sub_matches),
        (_, _) => unreachable!(),
    }?;

    Ok(())
}
