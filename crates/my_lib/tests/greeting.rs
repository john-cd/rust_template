//! Public API checks for the library greeting.
//!
//! TODO Reviewed ??-??-2026.

use my_lib::greeting;

/// Confirms the greeting function is reachable from an external crate.
#[test]
fn greeting_is_available_to_external_callers() {
    assert_eq!(greeting("Alice"), format!("Hello, Alice from {}!", env!("CARGO_PKG_NAME")));
}
