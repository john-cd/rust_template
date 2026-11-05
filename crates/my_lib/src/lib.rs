//! A starting point for a Rust library crate.
//!
//! TODO Reviewed ??-??-2026.

mod error;
mod greeting;

pub use error::MyError;
pub use error::Result;
pub use greeting::greeting;
