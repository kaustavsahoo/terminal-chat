use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_hello() {
    // Runs the compiled binary and asserts its stdout contains "hello" (case-insensitive-ish)
    let mut cmd = Command::cargo_bin("terminal-chat").expect("binary exists");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello").or(predicate::str::contains("Hello")));
}
