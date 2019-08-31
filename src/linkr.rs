extern crate clap;
use clap::*;

mod cmd_linkr;

fn main() {
    let app = App::new("linkr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`linkr` operates ranges on chromosomes and links of ranges")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_linkr::circos::make_subcommand());

    // Check which subcomamnd the user ran...
    let _res = match app.get_matches().subcommand() {
        ("circos", Some(sub_matches)) => cmd_linkr::circos::execute(sub_matches),
        (_, _) => unreachable!(),
    };
}
