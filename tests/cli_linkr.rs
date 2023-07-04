use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_circos() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("circos")
        .arg("tests/linkr/II.connect.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(stdout.contains("XIII 7947 6395"), "negative strand");
    assert!(!stdout.contains("fill_color"), "links");

    Ok(())
}

#[test]
fn command_circos_highlight() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("circos")
        .arg("tests/linkr/II.connect.tsv")
        .arg("--highlight")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 14);
    assert!(stdout.contains("fill_color"), "highlights");

    Ok(())
}

#[test]
fn command_sort() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("sort")
        .arg("tests/linkr/II.links.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 15);
    assert!(!stdout.contains("\nVI"), "chromosome II first");

    Ok(())
}

#[test]
fn command_filter() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/linkr/II.connect.tsv")
        .arg("-n")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);

    Ok(())
}

#[test]
fn command_filter_3() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/linkr/II.connect.tsv")
        .arg("-n")
        .arg("3")
        .arg("-r")
        .arg("0.99")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 1);
    assert!(!stdout.contains("VI("), "filtered links");

    Ok(())
}

#[test]
fn command_clean() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("clean")
        .arg("tests/linkr/II.sort.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 11);
    assert!(stdout.contains("892-4684"), "range exists");

    Ok(())
}

#[test]
fn command_clean_bundle() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("clean")
        .arg("tests/linkr/II.sort.tsv")
        .arg("--bundle")
        .arg("500")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(!stdout.contains("892-4684"), "original");
    assert!(stdout.contains("892-4685"), "bundled");

    Ok(())
}

#[test]
fn command_clean_merge() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("clean")
        .arg("tests/linkr/II.sort.tsv")
        .arg("-r")
        .arg("tests/linkr/II.merge.tsv")
        .arg("--verbose")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 8);
    assert!(!stdout.contains("892-4684"), "original");
    assert!(stdout.contains("892-4685"), "merged");

    Ok(())
}

#[test]
fn command_connect() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("linkr")?;
    let output = cmd
        .arg("connect")
        .arg("tests/linkr/II.clean.tsv")
        .arg("--verbose")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "multilateral links"
    );

    Ok(())
}
