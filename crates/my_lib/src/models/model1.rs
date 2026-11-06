//! Domain type used by the library.
//!
//! TODO Reviewed ??-??-2026.

/// Example domain type. Rename it and replace the fields.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Model1 {
    /// Display name.
    pub name: String,
}

impl Model1 {
    /// Creates a model with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
