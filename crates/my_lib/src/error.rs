//! Errors returned by the library.
//!
//! TODO Reviewed ??-??-2026.

/// The library's top-level error type.
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    /// An operating-system I/O operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Loading or deserializing the configuration failed.
    #[error("configuration error: {0}")]
    Config(#[from] config::ConfigError),

    /// Serializing the configuration failed.
    #[error("cannot serialize configuration: {0}")]
    Serialize(#[from] toml::ser::Error),
}

/// A result using the library's error type.
pub type Result<T> = std::result::Result<T, MyError>;
