//! Executable-level checks for development command failures.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

/// Reports a clear error when the selected external tool is unavailable.
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

/// Rejects an unsupported command before trying to launch a child process.
#[test]
fn invalid_command_is_rejected_before_dispatch() {
    let output = Command::new(env!("CARGO_BIN_EXE_tools"))
        .arg("unknown-command")
        .output()
        .expect("the development binary should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown-command"));
}
