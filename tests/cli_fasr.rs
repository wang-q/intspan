use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
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
fn command_cover() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("cover")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 16);
    assert!(stdout.contains("S288c"), "name list");
    assert!(stdout.contains("I"), "chr list");
    assert!(stdout.contains("13267-13287"), "runlist");

    // --name, --trim
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("cover")
        .arg("tests/fasr/example.fas")
        .arg("--name")
        .arg("S288c")
        .arg("--trim")
        .arg("10")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 3);
    assert!(!stdout.contains("S288c"), "name list");
    assert!(stdout.contains("I"), "chr list");
    assert!(stdout.contains("13277,184906"), "trimmed");

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

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("subset")
        .arg("tests/fasr/name.lst")
        .arg("tests/fasr/example.fas")
        .arg("--required")
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

    // --pair
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

    // --best
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("link")
        .arg("tests/fasr/example.fas")
        .arg("--best")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 9);
    assert_eq!(stdout.lines().next().unwrap().split_whitespace().count(), 2);

    Ok(())
}

#[test]
fn command_replace() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("replace")
        .arg("tests/fasr/replace.tsv")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 36);
    assert!(stdout.contains(">query.VIII(+)"));

    // fail
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("replace")
        .arg("tests/fasr/replace.fail.tsv")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert_eq!(stdout.lines().count(), 27);
    assert!(!stdout.contains("query"), "not replaced");
    assert!(stderr.contains("records"), "error message");

    // remove
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("replace")
        .arg("tests/fasr/replace.remove.tsv")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18);
    assert!(!stdout.contains("13267-13287"), "block removed");

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
fn command_split() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("split")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 27);

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("split")
        .arg("tests/fasr/example.fas")
        .arg("--simple")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains(">S288c\n"), "simple headers");
    assert!(!stdout.contains("I(+)"), "no positions");

    Ok(())
}

#[test]
fn command_split_to() -> anyhow::Result<()> {
    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("fasr")?;
    cmd.arg("split")
        .arg("tests/fasr/example.fas")
        .arg("--suffix")
        .arg(".tmp")
        .arg("--chr")
        .arg("-o")
        .arg(tempdir_str)
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    assert!(&tempdir.path().join("S288c.I.tmp").is_file());
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

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("consensus")
        .arg("tests/fasr/refine.fas")
        .arg("--outgroup")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 10);
    assert!(stdout.contains(">Spar"), "outgroup");

    Ok(())
}

#[test]
fn command_refine() -> anyhow::Result<()> {
    match which::which("clustaw") {
        Err(_) => return Ok(()),
        Ok(_) => {}
    }

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("refine")
        .arg("tests/fasr/refine.fas")
        .arg("--msa")
        .arg("clustalw")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18);
    assert!(stdout.contains("---"), "dashes added");

    Ok(())
}

#[test]
fn command_join() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("join")
        .arg("tests/fasr/S288cvsSpar.slice.fas")
        .arg("--name")
        .arg("Spar")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 5);
    assert!(
        stdout.lines().next().unwrap().contains(">Spar"),
        "Selected name first"
    );

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("join")
        .arg("tests/fasr/S288cvsRM11_1a.slice.fas")
        .arg("tests/fasr/S288cvsYJM789.slice.fas")
        .arg("tests/fasr/S288cvsSpar.slice.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 9);
    assert!(
        stdout.lines().next().unwrap().contains(">S288c."),
        "First name first"
    );

    Ok(())
}

#[test]
fn command_slice() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("slice")
        .arg("tests/fasr/slice.json")
        .arg("tests/fasr/slice.fas")
        .arg("--name")
        .arg("S288c")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 7);
    assert!(stdout.contains("13301-13400"), "sliced S288c");
    assert!(stdout.contains("2511-2636"), "sliced Spar");
    assert!(stdout.contains("\nTAGTCATCTCAG"), "sliced S288c seq");

    Ok(())
}

#[test]
fn command_stat() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("stat")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("0.192\t6\n"), "all together");

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("stat")
        .arg("tests/fasr/example.fas")
        .arg("--outgroup")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 4);
    assert!(stdout.contains("0.12\t3\n"), "exclude outgroup");

    Ok(())
}

#[test]
fn command_variation() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("variation")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 81);

    Ok(())
}

#[test]
fn command_filter() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/fasr/example.fas")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 27);

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/fasr/example.fas")
        .arg("--ge")
        .arg("30")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 18);

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("filter")
        .arg("tests/fasr/example.fas")
        .arg("--ge")
        .arg("30")
        .arg("--le")
        .arg("100")
        .arg("--name")
        .arg("S288c")
        .arg("--dash")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 9);
    assert!(stdout.contains("\nGCTAAAATATGAACG"), "no dash");

    Ok(())
}

#[test]
fn command_pl_p2m() -> anyhow::Result<()> {
    match which::which("clustaw") {
        Err(_) => return Ok(()),
        Ok(_) => {}
    }

    let tempdir = TempDir::new().unwrap();
    let tempdir_str = tempdir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("fasr")?;
    let output = cmd
        .arg("pl-p2m")
        .arg("tests/fasr/S288cvsRM11_1a.slice.fas")
        .arg("tests/fasr/S288cvsYJM789.slice.fas")
        .arg("tests/fasr/S288cvsSpar.slice.fas")
        .arg("-o")
        .arg(tempdir_str)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.lines().count(), 13);
    assert!(&tempdir.path().join("merge.json").is_file());
    assert!(&tempdir.path().join("join.subset.fas").is_file());

    tempdir.close()?;

    Ok(())
}
