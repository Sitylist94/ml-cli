use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_lists_available_commands() {
    let mut command = Command::cargo_bin("mlcli").unwrap();

    command
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("validate"));
}

#[test]
fn unknown_command_returns_an_error() {
    let mut command = Command::cargo_bin("mlcli").unwrap();

    command
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unrecognized subcommand 'unknown'",
        ));
}
