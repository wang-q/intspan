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

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("runlist")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .arg("--op")
        .arg("overlap")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "field count"
    );
    assert!(stdout.contains("I:1-100000"));
    assert!(!stdout.contains("Mito:1-85779"));

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

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("count")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("tests/rgr/S288c.rg")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        4,
        "field count"
    );
    assert!(stdout.contains("range\tcount"));
    assert!(stdout.contains("I:1-100000\t4"));
    assert!(stdout.contains("Mito:1-85779\t0"));

    Ok(())
}

#[test]
fn command_field() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("field")
        .arg("tests/Atha/chr.sizes")
        .arg("--chr")
        .arg("1")
        .arg("--start")
        .arg("2")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 5);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        1,
        "field count"
    );
    assert!(!stdout.contains("4\t18585056"));
    assert!(stdout.contains("4:18585056"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("field")
        .arg("tests/Atha/chr.sizes")
        .arg("--chr")
        .arg("1")
        .arg("--start")
        .arg("2")
        .arg("-H")
        .arg("-a")
        .arg("-s") // no effect
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 5);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "field count"
    );
    assert!(stdout.contains("30427671\trange"));
    assert!(!stdout.contains("1:30427671"));
    assert!(stdout.contains("4\t18585056"));
    assert!(stdout.contains("4:18585056"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("field")
        .arg("tests/spanr/NC_007942.gff")
        .arg("-H")
        .arg("--chr")
        .arg("1")
        .arg("--start")
        .arg("4")
        .arg("--end")
        .arg("5")
        .arg("--strand")
        .arg("7")
        .arg("--eq")
        .arg("3:tRNA")
        .arg("--ne")
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
    assert!(stdout.contains("NC_007942(-):13066-13138"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("field")
        .arg("tests/rgr/ctg.tsv")
        .arg("-H")
        .arg("-f")
        .arg("6,1")
        .arg("--chr")
        .arg("2")
        .arg("--start")
        .arg("3")
        .arg("--end")
        .arg("4")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "field count"
    );
    assert!(stdout.contains("ID\trange"));
    assert!(stdout.contains("ctg:I:2\tI:100001-230218"));

    Ok(())
}

#[test]
fn command_sort() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd.arg("sort").arg("tests/rgr/S288c.rg").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        1,
        "field count"
    );
    assert!(stdout.contains("S288c.I(-):190-200\nS288c"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("sort")
        .arg("tests/rgr/ctg.range.tsv")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "field count"
    );
    assert!(stdout.contains("Mito:1-85779\nlength"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("sort")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        3,
        "field count"
    );
    assert!(stdout.contains("range\n100000"));
    assert!(stdout.contains("I:1-100000\n130218"));

    Ok(())
}

#[test]
fn command_prop() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("prop")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/S288c.rg")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        2,
        "field count"
    );
    assert!(stdout.contains("I:1-100\t0.0000"));
    assert!(stdout.contains("II:21294-22075\t1.0000"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("prop")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        4,
        "field count"
    );
    assert!(stdout.contains("range\tprop"));
    assert!(stdout.contains("I:1-100000\t0.1301"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("prop")
        .arg("tests/rgr/intergenic.yml")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .arg("--prefix")
        .arg("--full")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        6,
        "field count"
    );
    assert!(stdout.contains("range\tintergenicProp\tintergenicLength\tintergenicSize"));
    assert!(stdout.contains("I:1-100000\t0.1301\t100000\t13011"));

    Ok(())
}
