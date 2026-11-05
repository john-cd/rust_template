//! Errors returned by the library.
//!
//! TODO Reviewed ??-??-????.

//

/// The library's top-level error type.
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    /// An operating-system I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// A result using the library's error type.
pub type Result<T> = std::result::Result<T, MyError>;
