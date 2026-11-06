//! Configuration loading and application settings.
//!
//! TODO Reviewed ??-??-2026.

/// Application settings. Replace the fields with your own.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Config {
    /// Name used when greeting.
    pub name: String,
}

impl Config {
    /// Creates a configuration with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
