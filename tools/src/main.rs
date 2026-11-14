//! Executable entry point for the workspace's development-only commands.
//!
//! TODO Reviewed ??-??-2026.

use std::process::ExitCode;

///
/// # Returns
/// A process status reflecting the outcome of the command.
fn main() -> ExitCode {
    tools::run()
}
