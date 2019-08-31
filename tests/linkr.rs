use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use tempfile::TempDir;

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linkr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_circos() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("circos")
        .arg("tests/linkr/II.connect.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 10);
    assert!(stdout.contains("XIII 7947 6395"), "negtive strand");
    assert!(!stdout.contains("fill_color"), "links");

    Ok(())
}

#[test]
fn command_circos_highlight() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("circos")
        .arg("tests/linkr/II.connect.tsv")
        .arg("--highlight")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 14);
    assert!(stdout.contains("fill_color"), "highlights");

    Ok(())
}
