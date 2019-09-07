use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_covered() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    let output = cmd
        .arg("covered")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 8);
    assert!(stdout.contains("pac4745_7148"), "original names");
    assert!(!stdout.contains("pac4745_7148:1"), "uncovered region");

    Ok(())
}

#[test]
fn command_covered_paf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    let output = cmd
        .arg("covered")
        .arg("tests/ovlpr/11_2.long.paf")
        .arg("--paf")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 15);
    assert!(stdout.contains("long/13141/0_10011"), "original names");
    assert!(!stdout.contains("long/13141/0_10011:1"), "uncovered region");

    Ok(())
}

#[test]
fn command_covered_longest() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    let output = cmd
        .arg("covered")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .arg("--longest")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 8);
    assert!(stdout.contains("pac4745_7148"), "original names");
    assert!(!stdout.contains("pac4745_7148:1"), "uncovered region");

    Ok(())
}

#[test]
fn command_covered_base() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    let output = cmd
        .arg("covered")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .arg("--base")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 98105);
    assert!(stdout.contains("pac4745_7148"), "original names");

    Ok(())
}

#[test]
fn command_covered_mean() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ovlpr")?;
    let output = cmd
        .arg("covered")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .arg("tests/ovlpr/1_4.pac.paf.ovlp.tsv")
        .arg("--mean")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 8);
    assert!(stdout.contains("pac4745_7148"), "original names");
    assert!(
        stdout.contains("pac1461_9030\t9030\t2.8"),
        "avoid duplicates"
    );

    Ok(())
}
