use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("recognized"));

    Ok(())
}

#[test]
fn command_name() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("name")
        .arg("tests/fasr/example.fas")
        .arg("-c")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("S288c\t3"), "count");
    assert!(stdout.contains("S288c\t3\nYJM789\t3\nRM11"), "name order");

    Ok(())
}

#[test]
fn command_maf2fas() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("maf2fas")
        .arg("tests/fasr/example.maf")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18);
    assert!(stdout.contains("S288c.VIII"), "name list");
    assert!(stdout.contains(":42072-42168"), "coordinate Transformed");

    Ok(())
}

#[test]
fn command_concat() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("concat")
        .arg("tests/fasr/example.fas")
        .arg("tests/fasr/name.lst")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(stdout.lines().next().unwrap().len(), 5); // >Spar
    assert_eq!(stdout.lines().last().unwrap().len(), 239); // >Spar
    assert!(stdout.contains("Spar"), "name list");
    assert!(!stdout.contains("S288c"), "name list");

    Ok(())
}

#[test]
fn command_concat_phylip() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("concat")
        .arg("tests/fasr/example.fas")
        .arg("tests/fasr/name.lst")
        .arg("--phylip")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert_eq!(
        stdout.lines().last().unwrap().len(),
        "YJM789".to_string().len() + 1 + 239
    );

    Ok(())
}
