use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use tempfile::TempDir;

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
    assert!(stdout.contains(":42072-42168"), "coordinate transformed");

    Ok(())
}

#[test]
fn command_axt2fas() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("axt2fas")
        .arg("tests/fasr/RM11_1a.chr.sizes")
        .arg("tests/fasr/example.axt")
        .arg("--qname")
        .arg("RM11_1a")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(stdout.contains("target.I(+)"), "name list");
    assert!(stdout.contains("RM11_1a.scaffold_14"), "name list");
    assert!(stdout.contains("(+):3634-3714"), "positive strand");
    assert!(stdout.contains("(-):22732-22852"), "coordinate transformed");

    Ok(())
}

#[test]
fn command_concat() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("concat")
        .arg("tests/fasr/name.lst")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(stdout.lines().next().unwrap().len(), 5); // >Spar
    assert_eq!(stdout.lines().last().unwrap().len(), 239);
    assert!(stdout.contains("Spar"), "name list");
    assert!(!stdout.contains("S288c"), "name list");

    Ok(())
}

#[test]
fn command_concat_phylip() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("concat")
        .arg("tests/fasr/name.lst")
        .arg("tests/fasr/example.fas")
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

#[test]
fn command_subset() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("subset")
        .arg("tests/fasr/name.lst")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 15);
    assert!(stdout.lines().next().unwrap().contains("Spar")); // >Spar.

    Ok(())
}

#[test]
fn command_link() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("link")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert_eq!(stdout.lines().next().unwrap().split_whitespace().count(), 4);

    Ok(())
}

#[test]
fn command_link_pair() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("link")
        .arg("tests/fasr/example.fas")
        .arg("--pair")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18);
    assert_eq!(stdout.lines().next().unwrap().split_whitespace().count(), 2);

    Ok(())
}

#[test]
fn command_check() -> anyhow::Result<()> {
    match which::which("samtools") {
        Err(_) => return Ok(()),
        Ok(_) => {}
    }

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("check")
        .arg("tests/fasr/NC_000932.fa")
        .arg("tests/fasr/A_tha.pair.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert!(stdout.lines().next().unwrap().contains("\tOK"));
    assert!(stdout.lines().last().unwrap().contains("\tFAILED"));

    // --name
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("check")
        .arg("tests/fasr/NC_000932.fa")
        .arg("tests/fasr/A_tha.pair.fas")
        .arg("--name")
        .arg("A_tha")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(stdout.lines().next().unwrap().contains("\tOK"));
    assert!(stdout.lines().last().unwrap().contains("\tOK"));

    Ok(())
}

#[test]
fn command_create() -> anyhow::Result<()> {
    match which::which("samtools") {
        Err(_) => return Ok(()),
        Ok(_) => {}
    }

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("create")
        .arg("tests/fasr/genome.fa")
        .arg("tests/fasr/I.connect.tsv")
        .arg("--name")
        .arg("S288c")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(stdout.contains("tgtgtgggtgtggtgtgg"), "revcom sequences");
    assert!(stdout.lines().next().unwrap().contains(">S288c."));

    Ok(())
}

#[test]
fn command_separate() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("separate")
        .arg("tests/fasr/example.fas")
        .arg("--rc")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 24);
    assert_eq!(
        stdout.lines().last().unwrap().len(),
        57,
        "length after remove dashes"
    );
    assert!(!stdout.contains("(-)"), "all strands are +");
    assert!(!stdout.contains("T-C"), "no dash, line 24");

    Ok(())
}

#[test]
fn command_separate_to() -> anyhow::Result<()> {
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("fasr")?;
    cmd.arg("separate")
        .arg("tests/fasr/example.fas")
        .arg("--suffix")
        .arg(".tmp")
        .arg("-o")
        .arg(tempdir_str)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("S288c.tmp").is_file());
    assert!(!&tempdir.path().join("YJM789.fasta").exists());

    tempdir.close()?;
    Ok(())
}

#[test]
fn command_consensus() -> anyhow::Result<()> {
    match which::which("spoa") {
        Err(_) => return Ok(()),
        Ok(_) => {}
    }

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("consensus")
        .arg("tests/fasr/refine.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains(">consensus\n"), "simple name");
    assert!(stdout.contains(">consensus.I("), "fas name");

    Ok(())
}
