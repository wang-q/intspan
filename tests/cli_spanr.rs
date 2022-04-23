use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use tempfile::TempDir;

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

#[test]
fn file_doesnt_be_needed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("test").arg("tests/spanr/S288c.chr.sizes");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

// #[test]
// fn file_doesnt_provided() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("spanr")?;
//     cmd.arg("genome");
//     cmd.assert().failure().stderr(predicate::str::contains(
//         "The following required arguments were not provided",
//     ));
//
//     Ok(())
// }

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("genome").arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not open"));

    Ok(())
}

#[test]
fn command_genome() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("genome").arg("tests/spanr/S288c.chr.sizes");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("I: 1-230218"));

    Ok(())
}

#[test]
fn command_some() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("some")
        .arg("tests/spanr/Atha.yml")
        .arg("tests/spanr/Atha.list")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 7 || stdout.lines().count() == 8);
    assert!(stdout.contains("AT2G01008"));
    assert!(!stdout.contains("AT2G01021"));

    Ok(())
}

#[test]
fn command_merge() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("merge")
        .arg("tests/spanr/I.yml")
        .arg("tests/spanr/II.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 5 || stdout.lines().count() == 6);
    assert!(stdout.contains("28547-29194"));
    assert!(stdout.contains("\nI:\n"));
    assert!(stdout.contains("\nII:\n"));

    // --all
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("merge")
        .arg("tests/spanr/I.yml")
        .arg("tests/spanr/II.other.yml")
        .arg("--all")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 5 || stdout.lines().count() == 6);
    assert!(stdout.contains("28547-29194"));
    assert!(stdout.contains("\nI:\n"));
    assert!(!stdout.contains("\nII:\n"));
    assert!(stdout.contains("\nII.other:\n"));

    Ok(())
}

#[test]
fn command_split() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("split")
        .arg("tests/spanr/I.II.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("28547-29194"));
    assert!(stdout.contains("---\nI: "));
    assert!(stdout.contains("---\nII: "));

    Ok(())
}

#[test]
fn command_split_to() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("split")
        .arg("tests/spanr/I.II.yml")
        .arg("-o")
        .arg(tempdir_str)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("II.yml").is_file());
    assert!(!&tempdir.path().join("I.II.yml").exists());

    tempdir.close()?;
    Ok(())
}

#[test]
fn command_stat() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("stat")
        .arg("tests/spanr/S288c.chr.sizes")
        .arg("tests/spanr/intergenic.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18, "line count");
    assert_eq!(
        stdout.lines().next().unwrap().split(',').count(),
        4,
        "field count"
    );
    assert!(stdout.contains("all,12071326,1059702,"));

    Ok(())
}

#[test]
fn command_stat_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("stat")
        .arg("tests/spanr/S288c.chr.sizes")
        .arg("tests/spanr/intergenic.yml")
        .arg("--all")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2, "line count");
    assert_eq!(
        stdout.lines().next().unwrap().split(',').count(),
        3,
        "field count"
    );
    assert!(!stdout.contains("all"));

    Ok(())
}

#[test]
fn command_statop() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("statop")
        .arg("tests/spanr/S288c.chr.sizes")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18, "line count");
    assert_eq!(
        stdout.lines().next().unwrap().split(',').count(),
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
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("statop")
        .arg("tests/spanr/S288c.chr.sizes")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--all")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2, "line count");
    assert_eq!(
        stdout.lines().next().unwrap().split(',').count(),
        7,
        "field count"
    );
    assert!(stdout.contains("36721"), "sum exists");
    assert!(stdout.contains(",repeatLength,"));
    assert!(!stdout.contains("\nI,"));
    assert!(!stdout.contains("\nXVI,"));

    Ok(())
}

#[test]
fn command_statop_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("statop")
        .arg("tests/spanr/S288c.chr.sizes")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--op")
        .arg("invalid")
        .arg("--all");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid IntSpan Op"));

    Ok(())
}

#[test]
fn command_combine() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("combine")
        .arg("tests/spanr/Atha.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 3 || stdout.lines().count() == 4);
    assert!(!stdout.contains("7232,7384"), "combined");

    // op
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("combine")
        .arg("tests/spanr/Atha.yml")
        .arg("--op")
        .arg("xor")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 3 || stdout.lines().count() == 4);
    assert!(stdout.contains("7233-7383"), "xor");

    // II
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("combine")
        .arg("tests/spanr/II.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert!(stdout.contains("21294-22075,"), "no changes");

    Ok(())
}

#[test]
fn command_compare() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("compare")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--op")
        .arg("intersect")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 17 || stdout.lines().count() == 18);
    assert!(stdout.contains("878539-878709"), "runlist exists");
    assert!(stdout.contains("\nI:"));
    assert!(stdout.contains("\nXVI:"));

    // union
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("compare")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--op")
        .arg("union")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 17 || stdout.lines().count() == 18);
    assert!(!stdout.contains("\"-\""), "no empty runlists");
    assert!(stdout.contains("\nI:"));
    assert!(stdout.contains("\nXVI:"));

    // xor
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("compare")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--op")
        .arg("xor")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 17 || stdout.lines().count() == 18);
    assert!(!stdout.contains("\"-\""), "no empty runlists");
    assert!(stdout.contains("\nI:"));
    assert!(stdout.contains("\nXVI:"));

    // m
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("compare")
        .arg("tests/spanr/I.II.yml")
        .arg("tests/spanr/repeat.yml")
        .arg("--op")
        .arg("intersect")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 35 || stdout.lines().count() == 36);

    // m3
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("compare")
        .arg("tests/spanr/I.II.yml")
        .arg("tests/spanr/I.yml")
        .arg("tests/spanr/II.yml")
        .arg("--op")
        .arg("intersect")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 7 || stdout.lines().count() == 8);
    assert!(!stdout.contains("13744-17133"), "all empty");

    Ok(())
}

#[test]
fn command_span() -> Result<(), Box<dyn std::error::Error>> {
    // cover
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("cover")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert!(stdout.contains("32316461-32398770"), "cover");

    // fill
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("fill")
        .arg("-n")
        .arg("1000")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert!(stdout.contains("32325076-32326613"), "newly emerged");
    assert_ne!(stdout.len() - stdout.replace(",", "").len(), 25, "original");
    assert_eq!(stdout.len() - stdout.replace(",", "").len(), 18, "new");

    // trim
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("trim")
        .arg("-n")
        .arg("200")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert_ne!(stdout.len() - stdout.replace(",", "").len(), 25, "original");
    assert_eq!(stdout.len() - stdout.replace(",", "").len(), 3, "new");

    // pad
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("pad")
        .arg("-n")
        .arg("2000")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert_ne!(stdout.len() - stdout.replace(",", "").len(), 25, "original");
    assert_eq!(stdout.len() - stdout.replace(",", "").len(), 6, "new");

    // excise
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("excise")
        .arg("-n")
        .arg("400")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert_ne!(stdout.len() - stdout.replace(",", "").len(), 25, "original");
    assert_eq!(stdout.len() - stdout.replace(",", "").len(), 3, "new");

    Ok(())
}

#[test]
fn command_span_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("span")
        .arg("tests/spanr/brca2.yml")
        .arg("--op")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid IntSpan Op"));

    Ok(())
}

#[test]
fn command_cover() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("cover")
        .arg("tests/spanr/S288c.ranges")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 3 || stdout.lines().count() == 4);
    assert!(!stdout.contains("S288c"), "species name");
    assert!(!stdout.contains("1-100"), "merged");
    assert!(stdout.contains("1-150"), "covered");

    // dazz
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("cover")
        .arg("tests/spanr/dazzname.ranges")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert!(stdout.contains("infile_0/1/0_514"), "chr name");
    assert!(stdout.contains("19-499"), "covered");

    Ok(())
}

#[test]
fn command_coverage() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("coverage")
        .arg("tests/spanr/S288c.ranges")
        .arg("-m")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 3 || stdout.lines().count() == 4);
    assert!(!stdout.contains("S288c"), "species name");
    assert!(!stdout.contains("1-150"), "coverage 1");
    assert!(stdout.contains("90-100"), "coverage 2");

    Ok(())
}

#[test]
fn command_coverage_detailed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("coverage")
        .arg("tests/spanr/S288c.ranges")
        .arg("-m")
        .arg("1")
        .arg("-d")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // eprintln!("stdout = {:#?}", stdout);

    assert!(stdout.lines().count() == 6 || stdout.lines().count() == 7);
    assert!(!stdout.contains("S288c"), "species name");
    assert!(stdout.contains("1-89"), "coverage 1");
    assert!(stdout.contains("90-100"), "coverage 2");
    assert!(stdout.contains("190-200"), "coverage 2");

    Ok(())
}

#[test]
fn command_gff() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("gff")
        .arg("tests/spanr/NC_007942.gff")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 2 || stdout.lines().count() == 3);
    assert!(stdout.contains("NC_007942"), "chromosomes exists");
    assert!(stdout.contains("1-152218"), "full chr runlist");

    Ok(())
}

#[test]
fn command_gff_merge() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("gff")
        .arg("tests/spanr/NC_007942.gff")
        .arg("--tag")
        .arg("CDS")
        .arg("-o")
        .arg(&tempdir.path().join("cds.yml"))
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("cds.yml").is_file());

    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("gff")
        .arg("tests/spanr/NC_007942.rm.gff")
        .arg("-o")
        .arg(&tempdir.path().join("repeat.yml"))
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("repeat.yml").is_file());

    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("merge")
        .arg(&tempdir.path().join("cds.yml"))
        .arg(&tempdir.path().join("repeat.yml"))
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.lines().count() == 5 || stdout.lines().count() == 6);
    assert!(stdout.contains("cds"));
    assert!(stdout.contains("repeat"));

    tempdir.close()?;
    Ok(())
}

#[test]
fn command_convert() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("convert")
        .arg("tests/spanr/repeat.yml")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 28);
    assert!(stdout.contains("II:327069-327703"), "first range");

    Ok(())
}

#[test]
fn command_range() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("range")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/S288c.ranges")
        .arg("--op")
        .arg("overlap")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 2);
    assert!(!stdout.contains("S288c"));
    assert!(stdout.contains("21294-22075"));

    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("range")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/S288c.ranges")
        .arg("--op")
        .arg("non-overlap")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("S288c"));
    assert!(!stdout.contains("21294-22075"));

    let mut cmd = Command::cargo_bin("spanr")?;
    let output = cmd
        .arg("range")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/S288c.ranges")
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
fn command_range_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("spanr")?;
    cmd.arg("range")
        .arg("tests/spanr/intergenic.yml")
        .arg("tests/spanr/S288c.ranges")
        .arg("--op")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid Range Op"));

    Ok(())
}
