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
    /// Constructs a usable initial model1 value with the defaults required by the model1 component.
    ///
    /// # Arguments
    /// * `name` - Human-readable value used to identify or display the selected model1.
    ///
    /// # Returns
    /// The self computed for the calling workflow.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
