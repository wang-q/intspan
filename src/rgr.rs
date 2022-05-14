extern crate clap;
use clap::*;

mod cmd_rgr;

fn main() -> std::io::Result<()> {
    let app = Command::new("rgr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`rgr` operates ranges (in tsv files)")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(cmd_rgr::replace::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("replace", sub_matches)) => cmd_rgr::replace::execute(sub_matches),
        _ => unreachable!(),
    }?;

    Ok(())
}

// TODO: `rgr field`
// TODO: `rgr annotate`
