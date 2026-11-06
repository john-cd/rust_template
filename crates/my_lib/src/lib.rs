//! A starting point for a Rust library crate.
//!
//! TODO Reviewed ??-??-2026.

pub mod config;
mod error;
mod greeting;
pub mod models;
pub mod services;

pub use error::MyError;
pub use error::Result;
pub use greeting::greeting;
