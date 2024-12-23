use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use tempfile::TempDir; // Run programs

#[test]
fn command_merge() -> anyhow::Result<()> {
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
fn command_sort() -> anyhow::Result<()> {
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
fn command_runlist() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("runlist")
        .arg("tests/rgr/intergenic.json")
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
        .arg("tests/rgr/intergenic.json")
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
        .arg("tests/rgr/intergenic.json")
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
        .arg("tests/rgr/intergenic.json")
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
fn command_runlist_invalid() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    cmd.arg("runlist")
        .arg("tests/rgr/intergenic.json")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid Op"));

    Ok(())
}

#[test]
fn command_field() -> anyhow::Result<()> {
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

    Ok(())
}

#[test]
fn command_count() -> anyhow::Result<()> {
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
fn command_span() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("span")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("trim")
        .arg("-n")
        .arg("10")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains("I:11-90"));
    assert!(stdout.contains("II:21304-22065"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("span")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("shift")
        .arg("-m")
        .arg("3p")
        .arg("-n")
        .arg("10")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains("I:11-110"));
    assert!(stdout.contains("S288c.I(-):180-190"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("span")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("flank")
        .arg("-m")
        .arg("3p")
        .arg("-n")
        .arg("10")
        .arg("-a")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert!(stdout.contains("I:1-100\tI:101-110"));
    assert!(stdout.contains("S288c.I(-):190-200|Species=Yeast\tS288c.I(-):180-189"));

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("span")
        .arg("tests/rgr/S288c.rg")
        .arg("--op")
        .arg("excise")
        .arg("-f")
        .arg("1")
        .arg("-n")
        .arg("20")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 6);
    assert_eq!(
        stdout
            .lines()
            .filter(|e| e.is_empty())
            .collect::<Vec<_>>()
            .len(),
        2,
        "empty lines"
    );

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("span")
        .arg("tests/rgr/ctg.range.tsv")
        .arg("-H")
        .arg("-f")
        .arg("3")
        .arg("--op")
        .arg("trim")
        .arg("-n")
        .arg("100")
        .arg("-m")
        .arg("5p")
        .arg("-a")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert_eq!(
        stdout.lines().next().unwrap().split('\t').count(),
        4,
        "field count"
    );
    assert!(stdout.contains("range\trg"));
    assert!(stdout.contains("I:1-100000\tI:101-100000"));
    assert!(stdout.contains("I:100001-230218\tI:100101-230218"));

    Ok(())
}

#[test]
fn command_prop() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("prop")
        .arg("tests/rgr/intergenic.json")
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
        .arg("tests/rgr/intergenic.json")
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
        .arg("tests/rgr/intergenic.json")
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

#[test]
fn command_pl_2rmp() -> anyhow::Result<()> {
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("rgr")?;
    let output = cmd
        .arg("pl-2rmp")
        .arg("tests/rgr/II.links.tsv")
        .arg("-o")
        .arg(tempdir_str.to_owned() + "/replaced.tsv")
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(stderr.lines().count() > 1);
    // assert!(&tempdir.path().join("replaced.tsv").is_file());

    tempdir.close()?;

    Ok(())
}
