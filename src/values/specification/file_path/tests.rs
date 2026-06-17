//! Verifies the bounded file-path value object.
//!
//! Bounded unit under test:
//! - `FilePath`
//!
//! Public interfaces verified:
//! - `FilePath::default`
//! - `FilePath::from`
//! - `FilePath::try_from`
//!
//! Logical paths covered:
//! - default file paths are empty
//! - valid file paths parse successfully
//! - missing file paths are rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::FilePath;
use crate::error::{Audience, Kind};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the default file path is empty.
#[test]
fn default_success() {
    let file_path = FilePath::default();
    assert_eq!(file_path.as_str(), "");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that infallible conversion preserves the supplied value.
#[test]
fn from_success() {
    let file_path = FilePath::from("some/file/path.file.txt");
    assert_eq!(file_path.as_str(), "some/file/path.file.txt");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a valid file path parses successfully.
#[test]
fn try_from_success() {
    let result = FilePath::try_from("C:\\Users\\johndoe\\Documents\\example.txt");
    let file_path = is_ok!(result);
    assert_eq!(
        file_path.as_str(),
        "C:\\Users\\johndoe\\Documents\\example.txt"
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that missing file paths are rejected.
#[test]
fn try_from_missing_value_error() {
    let result = FilePath::try_from("");
    let err = is_error!(result);
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(
        err.message
            .starts_with("The file path is required, although a value was not provided.")
    );
}
