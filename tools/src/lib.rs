//! Portable repository commands. Run with `cargo tools --help`.
//!
//! TODO Reviewed ??-??-2026.

/// Command definitions and dispatch for repository development tasks.
mod commands;

/// Parses and runs the selected development task.
pub use commands::run;
