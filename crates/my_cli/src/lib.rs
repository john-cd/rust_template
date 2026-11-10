//! Command-line application for the `my_cli` example.
//!
//! TODO Reviewed ??-??-2026.

/// Argument parsing and process-level behavior for the command-line application.
mod cli;
mod logging;

/// Parses arguments, runs the command, and returns its process exit code.
pub use cli::run;
