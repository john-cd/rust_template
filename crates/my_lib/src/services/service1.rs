//! Example application service operations for `Model1`.
//!
//! TODO Reviewed ??-??-2026.

use crate::models::Model1;

/// Example service operating on [`Model1`]. Rename it and add your logic.
#[derive(Debug, Clone, Default)]
pub struct Service1;

impl Service1 {
    /// Creates the stateless example service.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Formats the supplied model as `Model1(name)` using its display name.
    #[must_use]
    pub fn describe(&self, model: &Model1) -> String {
        format!("Model1({})", model.name)
    }
}

/// Checks the service's description of a domain model.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the description includes the model name in parentheses.
    #[test]
    fn describes_model() {
        assert_eq!(Service1::new().describe(&Model1::new("a")), "Model1(a)");
    }
}
