use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_txdb() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    let output = cmd
        .arg("txdb")
        .arg("--dir")
        .arg("tests/nwr/")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(std::path::Path::new("tests/nwr/taxonomy.sqlite").exists());
    assert_eq!(stdout.lines().count(), 8);

    Ok(())
}

#[test]
fn command_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    let output = cmd
        .arg("info")
        .arg("--dir")
        .arg("tests/nwr/")
        .arg("--tsv")
        .arg("Viruses")
        .arg("Bacillus phage bg1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert!(stdout.contains("10239\tViruses"), "first record");

    Ok(())
}

#[test]
fn command_lineage() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    let output = cmd
        .arg("lineage")
        .arg("--dir")
        .arg("tests/nwr/")
        .arg("Bacillus phage bg1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("Viruses\t10239"), "super kingdom");

    Ok(())
}

#[test]
fn command_restrict() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    let output = cmd
        .arg("restrict")
        .arg("--dir")
        .arg("tests/nwr/")
        .arg("Viruses")
        .arg("-c")
        .arg("2")
        .arg("-f")
        .arg("tests/nwr/taxon.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(stdout.contains("Actinophage JHJ-1\t12347"), "virus");

    Ok(())
}

#[test]
fn command_member() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    let output = cmd
        .arg("member")
        .arg("--dir")
        .arg("tests/nwr/")
        .arg("Synechococcus phage S")
        .arg("-r")
        .arg("species")
        .arg("-r")
        .arg("no rank")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 36);
    assert!(stdout.contains("375032\tSynechococcus phage S"), "virus");

    Ok(())
}
