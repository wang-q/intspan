use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn file_doesnt_be_needed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("test").arg("tests/resources/S288c.chr.sizes");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn command_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("test");
    cmd.assert().success().stdout(predicate::str::contains(
        "-30--21,-4-9,20-39,60-61,79-84,86,89-90,99",
    ));

    Ok(())
}

#[test]
fn file_doesnt_provided() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome");
    cmd.assert().failure().stderr(predicate::str::contains(
        "The following required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome").arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory").or(
            predicate::str::contains("The system cannot find the path specified"),
        ));

    Ok(())
}

#[test]
fn command_genome() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome").arg("tests/resources/S288c.chr.sizes");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("I: 1-230218"));

    Ok(())
}

#[test]
fn command_some() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("some")
        .arg("tests/resources/Atha.yml")
        .arg("tests/resources/Atha.list")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 7);
    assert!(stdout.contains("AT2G01008"));
    assert!(!stdout.contains("AT2G01021"));

    Ok(())
}

#[test]
fn command_merge() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("merge")
        .arg("tests/resources/I.yml")
        .arg("tests/resources/II.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 5);
    assert!(stdout.contains("28547-29194"));
    assert!(stdout.contains("\nI:"));
    assert!(stdout.contains("\nII:"));

    Ok(())
}
