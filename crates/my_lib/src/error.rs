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

    /// Reading a configuration file failed.
    #[error("could not read configuration file {path}: {source}")]
    ConfigFileRead {
        /// File that could not be read.
        path: std::path::PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },

    /// A configuration file exceeded the supported input size.
    #[error("configuration file {path} exceeds the {limit_bytes}-byte limit")]
    ConfigFileTooLarge {
        /// File that exceeded the limit.
        path: std::path::PathBuf,
        /// Maximum accepted file size.
        limit_bytes: usize,
    },

    /// A configuration file contained invalid UTF-8.
    #[error("configuration file {path} is not valid UTF-8: {source}")]
    ConfigFileEncoding {
        /// File with invalid text encoding.
        path: std::path::PathBuf,
        /// Underlying encoding error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// A configuration file could not be parsed as TOML.
    #[error("could not parse configuration file {path}: {source}")]
    ConfigFileParse {
        /// File that could not be parsed.
        path: std::path::PathBuf,
        /// Underlying TOML parsing error.
        #[source]
        source: toml::de::Error,
    },

    /// An atomic file write failed before or during replacement.
    #[error("could not {operation} atomically at {path}: {source}")]
    AtomicWrite {
        /// Destination file involved in the write.
        path: std::path::PathBuf,
        /// Filesystem operation that failed.
        operation: &'static str,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },

    /// Serializing the configuration failed.
    #[error("cannot serialize configuration: {0}")]
    Serialize(#[from] toml::ser::Error),
}

/// A result using the library's error type.
pub type Result<T> = std::result::Result<T, MyError>;
