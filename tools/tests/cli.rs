//! Executable-level checks for development command failures.
#![allow(missing_docs)]

use std::process::Command;

#[test]
fn missing_child_tool_reports_failure() {
    let output = Command::new(env!("CARGO_BIN_EXE_tools"))
        .arg("website")
        .env("PATH", "")
        .output()
        .expect("the development binary should start");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("could not run zola:"));
}

#[test]
fn invalid_command_is_rejected_before_dispatch() {
    let output = Command::new(env!("CARGO_BIN_EXE_tools"))
        .arg("unknown-command")
        .output()
        .expect("the development binary should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown-command"));
}
