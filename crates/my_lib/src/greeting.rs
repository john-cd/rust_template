//! Greeting behavior.
//!
//! TODO Reviewed ??-??-2026.

/// Returns a greeting for the provided name.
/// ```
/// assert_eq!(
///     my_lib::greeting("Alice"),
///     format!("Hello, Alice from {}!", env!("CARGO_PKG_NAME"))
/// );
/// ```
///
/// # Arguments
/// * `name` - Human-readable value used to identify or display the selected greeting.
///
/// # Returns
/// The formatted text produced for the caller.
#[must_use]
pub fn greeting(name: &str) -> String {
    tracing::debug!("preparing greeting");
    format!("Hello, {name} from {}!", env!("CARGO_PKG_NAME"))
}

/// Checks the library's greeting output.
#[cfg(test)]
mod tests {
    use super::greeting;

    /// Includes the supplied name in the greeting.
    #[test]
    fn greeting_includes_the_name() {
        assert_eq!(
            greeting("Alice"),
            format!("Hello, Alice from {}!", env!("CARGO_PKG_NAME"))
        );
    }
}
