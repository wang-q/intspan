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

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 8);
    assert!(stdout.contains("pac4745_7148"), "original names");
    assert!(!stdout.contains("pac4745_7148:1"), "uncovered region");

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

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 8);
    assert!(stdout.contains("pac4745_7148"), "original names");
    assert!(!stdout.contains("pac4745_7148:1"), "uncovered region");

    Ok(())
}
