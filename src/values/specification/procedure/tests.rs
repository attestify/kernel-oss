//! Tests for `Procedure`, covering repository/directory construction and validation.
//!
//! Bounded unit under test: `Procedure`.
//! Public interfaces verified: `try_new`.
//! Logical paths covered: valid repository-directory pairs and invalid repository/directory inputs.
//! Requirement validation points: standards-aligned validation for procedure repository metadata.

use super::Procedure;
use crate::error::{Audience, Kind};
use test_framework_oss::{is_error, is_ok, kernel_error_eq};

#[test]
/// Requirement validation: verifies `Procedure::try_new` accepts a valid repository and directory.
fn new_procedure_success() {
    let result = Procedure::try_new("https://example.com", "some/directory");
    let procedure = is_ok!(result);
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "some/directory");
}
#[test]
/// Requirement validation: verifies an empty directory normalizes to the root directory.
fn new_procedure_empty_directory_success() {
    let result = Procedure::try_new("https://example.com", "");
    let procedure = is_ok!(result);
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "/");
}
#[test]
/// Requirement validation: verifies the root directory is preserved.
fn new_procedure_root_directory_success() {
    let result = Procedure::try_new("https://example.com", "/");
    let procedure = is_ok!(result);
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "/");
}

#[test]
/// Requirement validation: verifies an empty repository is rejected.
fn new_procedure_invalid_repository_error() {
    let result = Procedure::try_new("", "some/directory");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .contains("'' is not a valid procedure location: ")
    );
}
#[test]
/// Requirement validation: verifies contiguous slashes are rejected in the directory path.
fn new_procedure_contiguous_slashes_error() {
    let result = Procedure::try_new("https://example.com", "a/bad/directory//path");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The directory path cannot contain contiguous forward slashes at position 16. Please remove the extra forward slash."
    );
}
#[test]
/// Requirement validation: verifies a leading slash is rejected in the directory path.
fn new_procedure_directory_path_starts_with_slash_error() {
    let result = Procedure::try_new("https://example.com", "/starts/with/slash");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The directory path cannot start or end with a forward slash at position."
    );
}
#[test]
/// Requirement validation: verifies a trailing slash is rejected in the directory path.
fn new_procedure_directory_path_ends_with_slash_error() {
    let result = Procedure::try_new("https://example.com", "ends/with/slash/");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The directory path cannot start or end with a forward slash at position."
    );
}
#[test]
/// Requirement validation: verifies invalid characters are rejected in the directory path.
fn new_procedure_directory_path_invalid_character_error() {
    let result = Procedure::try_new("https://example.com", "has@invalid/character");
    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "Invalid character at position 3: '@'. The directory path can only contain alphanumeric characters, underscores, and non-contiguous forward slashes."
    );
}

#[test]
/// Requirement validation: verifies a leading dash is rejected in the directory path.
fn new_procedure_directory_path_starts_with_dash_error() {
    let result = Procedure::try_new("https://example.com", "-starts/with/dash");

    kernel_error_eq!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The directory path cannot start or end with a dash."
    );
}

#[test]
/// Requirement validation: verifies a trailing dash is rejected in the directory path.
fn new_procedure_directory_path_ends_with_dash_error() {
    let result = Procedure::try_new("https://example.com", "ends/with/dash-");
    kernel_error_eq!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The directory path cannot start or end with a dash."
    );
}
