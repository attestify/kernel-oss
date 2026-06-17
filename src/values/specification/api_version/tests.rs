//! Verifies the bounded API version value object.
//!
//! Bounded unit under test:
//! - `APIVersion`
//!
//! Public interfaces verified:
//! - `APIVersion::new`
//! - `FromStr` parsing
//! - `APIVersion::major`
//! - `APIVersion::minor`
//! - `APIVersion::patch`
//! - `APIVersion::as_string`
//!
//! Logical paths covered:
//! - valid semver-style values parse successfully
//! - empty, malformed, and invalid numeric segments fail validation
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::APIVersion;
use crate::error::Audience;
use crate::error::Kind;
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that `APIVersion::new` stores the supplied major, minor, and patch
/// values and formats them as expected.
#[test]
fn new_api_version_success() {
    let api_version = APIVersion::new(1, 2, 3);
    assert_eq!(api_version.major, 1);
    assert_eq!(api_version.minor, 2);
    assert_eq!(api_version.patch, 3);
    assert_eq!(api_version.as_string(), "1.2.3");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that parsing a valid version string succeeds.
#[test]
fn from_str_api_version_success() {
    let api_version = is_ok!("1.2.3".parse::<APIVersion>());
    assert_eq!(api_version.major, 1);
    assert_eq!(api_version.minor, 2);
    assert_eq!(api_version.patch, 3);
    assert_eq!(api_version.as_string(), "1.2.3");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the component accessors return the stored version parts.
#[test]
fn major_minor_patch_accessors_success() {
    let api_version = APIVersion::new(4, 5, 6);

    assert_eq!(api_version.major(), 4);
    assert_eq!(api_version.minor(), 5);
    assert_eq!(api_version.patch(), 6);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty input is rejected during version parsing.
#[test]
fn new_api_version_empty_error() {
    let error = is_error!("  ".parse::<APIVersion>());
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "You provided an empty version. Please provide a version value which conforms to the Semver specification of 'major.minor.patch'."
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that malformed version input is rejected during parsing.
#[test]
fn new_api_version_invalid_input_error() {
    let error = is_error!("soemthing-invalid".parse::<APIVersion>());
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "Version string 'soemthing-invalid' is not in the format 'major.minor.patch'."
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a non-numeric major segment is rejected.
#[test]
fn new_api_version_invalid_major_error() {
    let error = is_error!("a.2.3".parse::<APIVersion>());
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "The major version 'a' from the supplied api version of 'a.2.3' is not a valid number. It must be a number between 0 and 255."
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a non-numeric minor segment is rejected.
#[test]
fn new_api_version_invalid_minor_error() {
    let error = is_error!("1.b.3".parse::<APIVersion>());
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "The minor version 'b' from the supplied api version of '1.b.3' is not a valid number.  It must be a number between 0 and 255"
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a non-numeric patch segment is rejected.
#[test]
fn new_api_version_invalid_patch_error() {
    let error = is_error!("1.2.c".parse::<APIVersion>());
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "The patch version 'c' from the supplied api version of '1.2.c' is not a valid number. It must be a number between 0 and 255"
    );
}
