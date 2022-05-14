use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_merge() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("merge")
        .arg("tests/rgr/II.links.tsv")
        .arg("--verbose")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains("892-4685"), "merged");

    Ok(())
}

#[test]
fn command_replace() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("replace")
        .arg("tests/rgr/1_4.ovlp.tsv")
        .arg("tests/rgr/1_4.replace.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 50);
    assert!(stdout.contains("pac6425_4471"), "original");
    assert!(!stdout.contains("falcon_read/12/0_4471"), "not replaced");

    Ok(())
}

#[test]
fn command_replace_reverse() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("replace")
        .arg("tests/rgr/1_4.ovlp.tsv")
        .arg("tests/rgr/1_4.replace.tsv")
        .arg("--reverse")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 50);
    assert!(!stdout.contains("pac6425_4471"), "original");
    assert!(stdout.contains("falcon_read/12/0_4471"), "replaced");

    Ok(())
}

#[test]
fn command_runlist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("runlist")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("overlap")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(!stdout.contains("S288c"));
    assert!(stdout.contains("21294-22075"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("runlist")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("non-overlap")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("S288c"));
    assert!(!stdout.contains("21294-22075"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("runlist")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("superset")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(!stdout.contains("S288c"));
    assert!(stdout.contains("21294-22075"));

    Ok(())
}

#[test]
fn command_runlist_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    cmd.arg("runlist")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid Op"));

    Ok(())
}

#[test]
fn command_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("count")
        .arg("tests/rgr/S288c.rg")
        .arg("tests/rgr/S288c.rg")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains("I:1-100\t2"));
    assert!(stdout.contains("21294-22075\t1"));

    Ok(())
}
