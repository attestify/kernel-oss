//! Verifies the bounded subject-id value object.
//!
//! Bounded unit under test:
//! - `SubjectId`
//!
//! Public interfaces verified:
//! - `SubjectId::new`
//!
//! Logical paths covered:
//! - valid subject IDs succeed
//! - empty subject IDs are rejected
//! - overly long subject IDs are rejected
//! - non-alphanumeric subject IDs are rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::SubjectId;
use crate::error::Kind;
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a valid subject ID parses successfully.
#[test]
fn new_subject_id_success() {
    let subject_id = SubjectId::new("subjectid");
    let subject_id = is_ok!(subject_id);
    assert_eq!(subject_id.value, "subjectid".to_string());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty subject IDs are rejected.
#[test]
fn new_subject_id_empty_error() {
    let subject_id = SubjectId::new("");
    let error = is_error!(subject_id);
    assert_eq!(error.message, "The Subject Id cannot be empty.");
    assert_eq!(error.kind, Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that overly long subject IDs are rejected.
#[test]
fn new_subject_id_max_length_error() {
    let subject_id = SubjectId::new("a".repeat(257).as_str());
    let error = is_error!(subject_id);
    assert_eq!(
        error.message,
        "The SubjectId exceeds the maximum length of 256 characters."
    );
    assert_eq!(error.kind, Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that non-alphanumeric subject IDs are rejected.
#[test]
fn new_subject_id_not_alphanumeric_error() {
    let subject_id = SubjectId::new("subject_id!");
    let error = is_error!(subject_id);
    assert_eq!(
        error.message,
        "The SubjectId can only contain alphanumeric characters."
    );
    assert_eq!(error.kind, Kind::InvalidInput);
}
