//! Executable entry point for the workspace's development-only commands.
//!
//! TODO Reviewed ??-??-2026.

use std::process::ExitCode;

/// Starts the tools command and returns its process exit status.
fn main() -> ExitCode {
    tools::run()
}
