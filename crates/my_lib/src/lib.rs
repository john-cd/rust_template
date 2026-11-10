//! A starting point for a Rust library crate.
//!
//! TODO Reviewed ??-??-2026.

/// Configuration loading and settings for library consumers.
pub mod config;
/// Error types shared by the library's public operations.
mod error;
/// Greeting behavior exposed by this starter library.
mod greeting;
/// Domain types used by the library.
pub mod models;
/// Application operations and use cases.
pub mod services;

/// The library's top-level error type.
pub use error::MyError;
/// A result type using [`MyError`].
pub use error::Result;
/// Returns a greeting for a name.
pub use greeting::greeting;
