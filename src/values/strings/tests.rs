//! Verifies shared string normalization helpers.
//!
//! Bounded unit under test:
//! - `remove_all_whitespace`
//!
//! Public interfaces verified:
//! - `remove_all_whitespace`
//!
//! Logical paths covered:
//! - whitespace is removed from mixed input
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::remove_all_whitespace;

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that all whitespace characters are removed from the input string.
#[test]
fn remove_all_whitespace_success() {
    let input = "  a     b    c d e    f  ";
    let expected = "abcdef";
    let result = remove_all_whitespace(input);
    assert_eq!(result, expected);
}
