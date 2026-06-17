//! Verifies the bounded short-description value object.
//!
//! Bounded unit under test:
//! - `ShortDescription`
//!
//! Public interfaces verified:
//! - `ShortDescription::try_from`
//!
//! Logical paths covered:
//! - valid short descriptions succeed
//! - empty short descriptions are rejected
//! - overly long short descriptions are rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::ShortDescription;
use crate::error::{Audience, Kind};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a valid short description parses successfully.
#[test]
fn new_short_description_success() {
    let result = ShortDescription::try_from("This is a short description.");
    let result = is_ok!(result);
    assert_eq!(result.value, "This is a short description.");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty short descriptions are rejected.
#[test]
fn new_short_description_empty_error() {
    let result = ShortDescription::try_from("");
    let err = is_error!(result);
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(
        err.message
            .starts_with("There is an issue with your short description: ")
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that short descriptions over the maximum length are rejected.
#[test]
fn short_description_too_long_error() {
    let max_length: usize = 255;
    let short_description = "a".repeat(max_length + 1);
    let result = ShortDescription::try_from(short_description.as_str());

    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("There is an issue with your short description: The short description provided is too long. A description must have a value with at most 255 characters."));
}
