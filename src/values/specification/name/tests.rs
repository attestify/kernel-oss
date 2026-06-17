//! Tests for `Name`, covering normalization, validation, and empty-input handling.
//!
//! Bounded unit under test: `Name`.
//! Public interfaces verified: `try_from`.
//! Logical paths covered: valid normalization, leading/trailing dash rejection, whitespace
//! rejection, invalid character rejection, and empty input.
//! Requirement validation points: standards-aligned behavior for the name value object.

use super::Name;
use crate::error::{Audience, Kind};
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies `Name::try_from` accepts a valid name.
fn new_name_success() {
    let result = Name::try_from("test-name");

    let name = is_ok!(result);
    assert_eq!(name.value, "test-name");
}
#[test]
/// Requirement validation: verifies `Name::try_from` normalizes uppercase letters.
fn lowercase_capital_letters_success() {
    let result = Name::try_from("Test-Name");

    let name = is_ok!(result);
    assert_eq!(name.value, "test-name");
}
#[test]
/// Requirement validation: verifies leading dashes are rejected.
fn leading_dash_error() {
    let result = Name::try_from("-test-name");

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The name cannot start or end with a dash, and you provided '-test-name'.".to_string()
    );
}
#[test]
/// Requirement validation: verifies trailing dashes are rejected.
fn trailing_dash_error() {
    let result = Name::try_from("test-name-");

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The name cannot start or end with a dash, and you provided 'test-name-'.".to_string()
    );
}
#[test]
/// Requirement validation: verifies spaces are rejected in names.
fn spaces_error() {
    let result = Name::try_from("some test name");

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "You provided the name 'some test name' which contains invalid characters. A name must contain only alphanumeric characters and dashes.".to_string());
}
#[test]
/// Requirement validation: verifies non-dash special characters are rejected in names.
fn special_character_error() {
    let result = Name::try_from("some_test_name");

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "You provided the name 'some_test_name' which contains invalid characters. A name must contain only alphanumeric characters and dashes.".to_string());
}
#[test]
/// Requirement validation: verifies empty names are rejected.
fn empty_name_error() {
    let result = Name::try_from("     ");

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "You provided an empty name. A name must contain at least one character.".to_string()
    )
}
