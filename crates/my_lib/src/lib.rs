//! A starting point for a Rust library crate.
// TODO Reviewed ??-??-????.

mod error;
mod greeting;

pub use error::MyError;
pub use error::Result;
pub use greeting::greeting;
