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
        .subcommand(cmd_fasr::check::make_subcommand())
        .subcommand(cmd_fasr::concat::make_subcommand())
        .subcommand(cmd_fasr::create::make_subcommand())
        .subcommand(cmd_fasr::link::make_subcommand())
        .subcommand(cmd_fasr::maf2fas::make_subcommand())
        .subcommand(cmd_fasr::name::make_subcommand())
        .subcommand(cmd_fasr::separate::make_subcommand())
        .subcommand(cmd_fasr::subset::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("check", sub_matches)) => cmd_fasr::check::execute(sub_matches),
        Some(("concat", sub_matches)) => cmd_fasr::concat::execute(sub_matches),
        Some(("create", sub_matches)) => cmd_fasr::create::execute(sub_matches),
        Some(("link", sub_matches)) => cmd_fasr::link::execute(sub_matches),
        Some(("maf2fas", sub_matches)) => cmd_fasr::maf2fas::execute(sub_matches),
        Some(("name", sub_matches)) => cmd_fasr::name::execute(sub_matches),
        Some(("separate", sub_matches)) => cmd_fasr::separate::execute(sub_matches),
        Some(("subset", sub_matches)) => cmd_fasr::subset::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}

// TODO: fasr link --best
// TODO: replace samtools
// TODO: add more tools
