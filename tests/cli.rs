use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use tempfile::TempDir;

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
    assert!(stdout.contains("\nI:\n"));
    assert!(stdout.contains("\nII:\n"));

    Ok(())
}

#[test]
fn command_split() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("split")
        .arg("tests/resources/I.II.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 4);
    assert!(stdout.contains("28547-29194"));
    assert!(stdout.contains("---\nI: "));
    assert!(stdout.contains("---\nII: "));

    Ok(())
}

#[test]
fn command_split_to() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("split")
        .arg("tests/resources/I.II.yml")
        .arg("-o")
        .arg(tempdir_str)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("II.yml").is_file());
    assert!(!&tempdir.path().join("I.II.yml").exists());

    Ok(())
}

#[test]
fn command_stat() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("stat")
        .arg("tests/resources/S288c.chr.sizes")
        .arg("tests/resources/intergenic.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 18, "line count");
    assert_eq!(
        stdout
            .lines()
            .next()
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>()
            .len(),
        4,
        "field count"
    );
    assert!(stdout.contains("all,12071326,1059702,"));

    Ok(())
}

#[test]
fn command_stat_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("stat")
        .arg("tests/resources/S288c.chr.sizes")
        .arg("tests/resources/intergenic.yml")
        .arg("--all")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 2, "line count");
    assert_eq!(
        stdout
            .lines()
            .next()
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>()
            .len(),
        3,
        "field count"
    );
    assert!(!stdout.contains("all"));

    Ok(())
}

#[test]
fn command_statop() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("statop")
        .arg("tests/resources/S288c.chr.sizes")
        .arg("tests/resources/intergenic.yml")
        .arg("tests/resources/repeat.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 18, "line count");
    assert_eq!(
        stdout
            .lines()
            .next()
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>()
            .len(),
        8,
        "field count"
    );
    assert!(stdout.contains("36721"), "sum exists");
    assert!(stdout.contains(",repeatLength,"));
    assert!(stdout.contains("\nI,"));
    assert!(stdout.contains("\nXVI,"));

    Ok(())
}

#[test]
fn command_statop_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output = cmd
        .arg("statop")
        .arg("tests/resources/S288c.chr.sizes")
        .arg("tests/resources/intergenic.yml")
        .arg("tests/resources/repeat.yml")
        .arg("--all")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().collect::<Vec<_>>().len(), 2, "line count");
    assert_eq!(
        stdout
            .lines()
            .next()
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>()
            .len(),
        7,
        "field count"
    );
    assert!(stdout.contains("36721"), "sum exists");
    assert!(stdout.contains(",repeatLength,"));
    assert!(!stdout.contains("\nI,"));
    assert!(!stdout.contains("\nXVI,"));

    Ok(())
}
