extern crate clap;
use clap::*;

mod cmd_linkr;

fn main() -> std::io::Result<()> {
    let app = App::new("linkr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`linkr` operates ranges on chromosomes and links of ranges")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_linkr::circos::make_subcommand())
        .subcommand(cmd_linkr::sort::make_subcommand())
        .subcommand(cmd_linkr::merge::make_subcommand())
        .subcommand(cmd_linkr::filter::make_subcommand())
        .subcommand(cmd_linkr::clean::make_subcommand())
        .subcommand(cmd_linkr::connect::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        ("circos", Some(sub_matches)) => cmd_linkr::circos::execute(sub_matches),
        ("sort", Some(sub_matches)) => cmd_linkr::sort::execute(sub_matches),
        ("merge", Some(sub_matches)) => cmd_linkr::merge::execute(sub_matches),
        ("filter", Some(sub_matches)) => cmd_linkr::filter::execute(sub_matches),
        ("clean", Some(sub_matches)) => cmd_linkr::clean::execute(sub_matches),
        ("connect", Some(sub_matches)) => cmd_linkr::connect::execute(sub_matches),
        (_, _) => unreachable!(),
    }?;

    Ok(())
}
