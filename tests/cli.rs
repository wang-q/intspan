use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("which wasn't expected, or isn't valid in this context"));

    Ok(())
}

#[test]
fn file_doesnt_be_needed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("test")
        .arg("tests/resources/S288c.chr.sizes");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("which wasn't expected, or isn't valid in this context"));

    Ok(())
}

#[test]
fn command_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("-30--21,-4-9,20-39,60-61,79-84,86,89-90,99"));

    Ok(())
}

#[test]
fn file_doesnt_provided() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments were not provided"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome")
        .arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn command_genome() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("genome")
        .arg("tests/resources/S288c.chr.sizes");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("I: 1-230218"));

    Ok(())
}
