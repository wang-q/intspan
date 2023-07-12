extern crate clap;
use clap::*;

mod cmd_fasr;

fn main() -> anyhow::Result<()> {
    let app = Command::new("fasr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`fasr` operates block fasta files")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(ColorChoice::Auto)
        .subcommand(cmd_fasr::name::make_subcommand())
        .subcommand(cmd_fasr::maf2fas::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("maf2fas", sub_matches)) => cmd_fasr::maf2fas::execute(sub_matches),
        Some(("name", sub_matches)) => cmd_fasr::name::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}
