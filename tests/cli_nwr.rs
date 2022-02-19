use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn command_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("nwr")?;
    cmd.arg("foobar");
    cmd.assert().failure().stderr(predicate::str::contains(
        "which wasn't expected, or isn't valid in this context",
    ));

    Ok(())
}

// #[test]
// fn command_size() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("nwr")?;
//     let output = cmd.arg("size").arg("tests/nwr/ufasta.fa").output().unwrap();
//     let stdout = String::from_utf8(output.stdout).unwrap();
//
//     assert_eq!(stdout.lines().count(), 50);
//     assert!(stdout.contains("read0\t359"), "read0");
//     assert!(stdout.contains("read1\t106"), "read1");
//
//     let mut sum = 0;
//     for line in stdout.lines() {
//         let fields: Vec<&str> = line.split('\t').collect();
//         if fields.len() == 2 {
//             sum += fields[1].parse::<i32>().unwrap();
//         }
//     }
//     assert_eq!(sum, 9317, "sum length");
//
//     Ok(())
// }
