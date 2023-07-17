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
        .subcommand(cmd_fasr::axt2fas::make_subcommand())
        .subcommand(cmd_fasr::check::make_subcommand())
        .subcommand(cmd_fasr::concat::make_subcommand())
        .subcommand(cmd_fasr::consensus::make_subcommand())
        .subcommand(cmd_fasr::cover::make_subcommand())
        .subcommand(cmd_fasr::create::make_subcommand())
        .subcommand(cmd_fasr::join::make_subcommand())
        .subcommand(cmd_fasr::link::make_subcommand())
        .subcommand(cmd_fasr::maf2fas::make_subcommand())
        .subcommand(cmd_fasr::name::make_subcommand())
        .subcommand(cmd_fasr::separate::make_subcommand())
        .subcommand(cmd_fasr::split::make_subcommand())
        .subcommand(cmd_fasr::stat::make_subcommand())
        .subcommand(cmd_fasr::subset::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("axt2fas", sub_matches)) => cmd_fasr::axt2fas::execute(sub_matches),
        Some(("check", sub_matches)) => cmd_fasr::check::execute(sub_matches),
        Some(("concat", sub_matches)) => cmd_fasr::concat::execute(sub_matches),
        Some(("consensus", sub_matches)) => cmd_fasr::consensus::execute(sub_matches),
        Some(("cover", sub_matches)) => cmd_fasr::cover::execute(sub_matches),
        Some(("create", sub_matches)) => cmd_fasr::create::execute(sub_matches),
        Some(("join", sub_matches)) => cmd_fasr::join::execute(sub_matches),
        Some(("link", sub_matches)) => cmd_fasr::link::execute(sub_matches),
        Some(("maf2fas", sub_matches)) => cmd_fasr::maf2fas::execute(sub_matches),
        Some(("name", sub_matches)) => cmd_fasr::name::execute(sub_matches),
        Some(("separate", sub_matches)) => cmd_fasr::separate::execute(sub_matches),
        Some(("split", sub_matches)) => cmd_fasr::split::execute(sub_matches),
        Some(("stat", sub_matches)) => cmd_fasr::stat::execute(sub_matches),
        Some(("subset", sub_matches)) => cmd_fasr::subset::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}

// TODO: replace samtools
// TODO: add more tools
// TODO: simple - replace
// TODO: hard - refine, slice, vars, xlsx; need find_islands()
// TODO: fasr kb - scripts for join pairwise alignments p2m
// TODO: lav2fas
// TODO: paf2fas
// TODO: vcf
// TODO: filter
