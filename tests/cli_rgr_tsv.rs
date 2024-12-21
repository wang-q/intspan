use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_invalid() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    cmd.arg("foobar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("recognized"));

    Ok(())
}

#[test]
fn command_replace() -> anyhow::Result<()> {
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
fn command_replace_reverse() -> anyhow::Result<()> {
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
fn command_md() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("md")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("--num")
        .arg("-c")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 5);
    assert!(
        stdout.contains("| -----: | :--------: | --------------- |"),
        "separator"
    );
    assert!(stdout.contains("| 130218 |  ctg:I:2   | I:100001-230218 |"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("md")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("--fmt")
        .arg("--digits")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 5);
    assert!(
        stdout.contains("| ---------: | ---------- | --------------- |"),
        "separator"
    );
    assert!(stdout.contains("| 130,218.00 | ctg:I:2    | I:100001-230218 |"));

    Ok(())
}

#[test]
fn command_dedup() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("dedup")
        .arg("tests/rgr/ctg.tsv")
        .arg("tests/rgr/ctg.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("dedup")
        .arg("tests/rgr/ctg.tsv")
        .arg("-f")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert!(!stdout.contains("ctg:I:2\tI"));

    Ok(())
}

#[test]
fn command_filter() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/spanr/NC_007942.gff")
        .arg("-H")
        .arg("--str-eq")
        .arg("3:tRNA")
        .arg("--str-ne")
        .arg("7:+")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 27);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        1,
        "field count"
    );
    assert!(stdout.contains("13066\t13138"));

    Ok(())
}

#[test]
fn command_select() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("select")
        .arg("tests/rgr/ctg.tsv")
        .arg("-H")
        .arg("-f")
        .arg("6,1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        2,
        "field count"
    );
    assert!(stdout.contains("length\tID"));
    assert!(stdout.contains("130218\tctg:I:2"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("select")
        .arg("tests/rgr/ctg.tsv")
        .arg("-H")
        .arg("-f")
        .arg("ID,1")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        2,
        "field count"
    );
    assert!(stdout.contains("ID\tID"));
    assert!(stdout.contains("ctg:I:2\tctg:I:2"));

    Ok(())
}
