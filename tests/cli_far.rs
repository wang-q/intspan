use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("far")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

// #[test]
// fn command_size() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("far")?;
//     let output = cmd
//         .arg("size")
//         .arg("tests/far/ufasta.fa")
//         .output()
//         .unwrap();
//     let stdout = String::from_utf8(output.stdout).unwrap();
//
//     assert_eq!(stdout.lines().count(), 46);
//     assert!(stdout.contains("read0\t359"), "first read");
//
//     Ok(())
// }
