//! Verifies the bounded description value object.
//!
//! Bounded unit under test:
//! - `Description`
//!
//! Public interfaces verified:
//! - `Description::try_from`
//!
//! Logical paths covered:
//! - valid descriptions succeed
//! - leading/trailing whitespace is trimmed
//! - empty descriptions are rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::Description;
use crate::error::Audience;
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a non-empty description parses successfully.
#[test]
fn new_description_success() {
    let result = Description::try_from("This is a description.");
    is_ok!(result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that surrounding whitespace is trimmed from descriptions.
#[test]
fn new_description_trim_whitespace_success() {
    let result = Description::try_from("    Some Description     ");
    let result = is_ok!(result);
    assert_eq!(result.value, "Some Description");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that tab characters are trimmed from descriptions.
#[test]
fn new_description_trim_tabs_success() {
    let result = Description::try_from("\t\t\t\tSome Description\t\t\t\t");
    let result = is_ok!(result);
    assert_eq!(result.value, "Some Description");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that newline characters are trimmed from descriptions.
#[test]
fn new_description_trim_newlines_success() {
    let result = Description::try_from("\n\n\n\nSome Description\n\n\n\n");
    let result = is_ok!(result);
    assert_eq!(result.value, "Some Description");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that carriage returns are trimmed from descriptions.
#[test]
fn new_description_trim_carriage_return_success() {
    let result = Description::try_from("\r\r\r\rSome Description\r\r\r\r");
    let result = is_ok!(result);
    assert_eq!(result.value, "Some Description");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty descriptions are rejected.
#[test]
fn new_description_empty_error() {
    let result = Description::try_from("  ");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, crate::error::Kind::InvalidInput);
    assert_eq!(
        error.message,
        "You provided an empty description. A description must have a value with at least one character."
    );
}
