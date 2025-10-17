use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_echor_with_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_echor_with_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success().stdout("hello\n");
}

#[test]
fn test_echor_with_omit_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").arg("-n").assert().success().stdout("hello");
}
