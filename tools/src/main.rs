//! Executable entry point for the workspace's development-only commands.
//!
//! TODO Reviewed ??-??-2026.

use std::process::ExitCode;

/// Runs the executable command and returns its exit status to the operating system.
fn main() -> ExitCode {
    tools::run()
}
