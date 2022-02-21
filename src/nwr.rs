extern crate clap;
use clap::*;

mod cmd_nwr;

fn main() -> std::io::Result<()> {
    let app = App::new("nwr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("`nwr` is a lightweight tool for newick and taxonomy")
        .global_setting(AppSettings::PropagateVersion)
        .subcommand(cmd_nwr::append::make_subcommand())
        .subcommand(cmd_nwr::download::make_subcommand())
        .subcommand(cmd_nwr::info::make_subcommand())
        .subcommand(cmd_nwr::lineage::make_subcommand())
        .subcommand(cmd_nwr::member::make_subcommand())
        .subcommand(cmd_nwr::restrict::make_subcommand())
        .subcommand(cmd_nwr::txdb::make_subcommand());

    // Check which subcomamnd the user ran...
    match app.get_matches().subcommand() {
        Some(("append", sub_matches)) => cmd_nwr::append::execute(sub_matches),
        Some(("download", sub_matches)) => cmd_nwr::download::execute(sub_matches),
        Some(("info", sub_matches)) => cmd_nwr::info::execute(sub_matches),
        Some(("lineage", sub_matches)) => cmd_nwr::lineage::execute(sub_matches),
        Some(("member", sub_matches)) => cmd_nwr::member::execute(sub_matches),
        Some(("restrict", sub_matches)) => cmd_nwr::restrict::execute(sub_matches),
        Some(("txdb", sub_matches)) => cmd_nwr::txdb::execute(sub_matches),
        _ => unreachable!(),
    }
    .unwrap();

    Ok(())
}
