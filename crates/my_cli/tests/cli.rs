//! Executable-level checks for the `my_cli` command-line contract.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

#[test]
fn cli_greeting_smoke_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_my_cli"))
        .arg("Alice")
        .output()
        .expect("CLI should start successfully");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, Alice from my_lib!\n");
    assert_eq!(output.stderr, Vec::<u8>::new());
}

#[test]
fn cli_uses_documented_default_name() {
    let output = Command::new(env!("CARGO_BIN_EXE_my_cli"))
        .output()
        .expect("CLI should start successfully");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world from my_lib!\n");
}

#[test]
fn cli_help_and_version_succeed() {
    for argument in ["--help", "--version"] {
        let output = Command::new(env!("CARGO_BIN_EXE_my_cli"))
            .arg(argument)
            .output()
            .expect("CLI should start successfully");

        assert!(output.status.success(), "{argument}");
        assert!(!output.stdout.is_empty(), "{argument}");
        assert!(output.stderr.is_empty(), "{argument}");
    }
}

#[test]
fn cli_rejects_unexpected_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_my_cli"))
        .args(["Alice", "Bob"])
        .output()
        .expect("CLI should start successfully");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, Vec::<u8>::new());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument"));
}
