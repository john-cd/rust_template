//! Executable entry point for the `my_cli` application.
//!
//! TODO Reviewed ??-??-2026.

/// Runs the executable command and returns its exit status to the operating system.
fn main() -> std::process::ExitCode {
    my_cli::run()
}
