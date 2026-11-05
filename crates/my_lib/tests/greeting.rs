//! Public API checks for the library greeting.
//!
//! TODO Reviewed ??-??-2026.

use my_lib::greeting;

#[test]
fn greeting_is_available_to_external_callers() {
    assert_eq!(greeting("Alice"), "Hello, Alice from my_lib!");
}
