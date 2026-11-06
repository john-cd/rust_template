//! Service 1.
//!
//! TODO Reviewed ??-??-2026.

use crate::models::Model1;

/// Example service operating on [`Model1`]. Rename it and add your logic.
#[derive(Debug, Clone, Default)]
pub struct Service1;

impl Service1 {
    /// Creates the service.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Returns a description of the model.
    #[must_use]
    pub fn describe(&self, model: &Model1) -> String {
        format!("Model1({})", model.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_model() {
        assert_eq!(Service1::new().describe(&Model1::new("a")), "Model1(a)");
    }
}
