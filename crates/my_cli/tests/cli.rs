//! Executable-level checks for the `my_cli` command-line contract.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

/// Prints a greeting for a supplied name with no diagnostic output.
#[test]
fn cli_greeting_smoke_test() {
    let output = Command::new(env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME"))))
        .arg("Alice")
        .output()
        .expect("CLI should start successfully");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        format!("Hello, Alice from {}!\n", env!("CARGO_PKG_NAME")).as_bytes()
    );
    assert_eq!(output.stderr, Vec::<u8>::new());
}

/// Uses the documented default name when no positional argument is supplied.
#[test]
fn cli_uses_documented_default_name() {
    let output = Command::new(env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME"))))
        .output()
        .expect("CLI should start successfully");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        format!("Hello, world from {}!\n", env!("CARGO_PKG_NAME")).as_bytes()
    );
}

/// Provides successful help and version responses.
#[test]
fn cli_help_and_version_succeed() {
    for argument in ["--help", "--version"] {
        let output = Command::new(env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME"))))
            .arg(argument)
            .output()
            .expect("CLI should start successfully");

        assert!(output.status.success(), "{argument}");
        assert!(!output.stdout.is_empty(), "{argument}");
        assert!(output.stderr.is_empty(), "{argument}");
    }
}

/// Rejects extra positional arguments with Clap's usage error status.
#[test]
fn cli_rejects_unexpected_arguments() {
    let output = Command::new(env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME"))))
        .args(["Alice", "Bob"])
        .output()
        .expect("CLI should start successfully");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, Vec::<u8>::new());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
}
