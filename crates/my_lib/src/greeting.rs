//! Greeting behavior.
//!
//! TODO Reviewed ??-??-????.

/// Returns a greeting for the provided name.
///
/// ```
/// assert_eq!(my_lib::greeting("Alice"), "Hello, Alice from my_lib!");
/// ```
#[must_use]
pub fn greeting(name: &str) -> String {
    format!("Hello, {name} from my_lib!")
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_includes_the_name() {
        assert_eq!(greeting("Alice"), "Hello, Alice from my_lib!");
    }
}
