//! Tests for assurance-procedure `Artifact`, covering construction and validation behavior.
//!
//! Bounded unit under test: `assurance_procedure::Artifact`.
//! Public interfaces verified: `new`.
//! Logical paths covered: successful construction and invalid name, description, and metadata
//! inputs.
//! Requirement validation points: standards-aligned artifact validation for assurance procedures.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::artifact::Artifact;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies `Artifact::new` accepts valid inputs.
fn new_artifact_success() {
    let artifact = Artifact::new(
        "artifact",
        "artifact description",
        &[("artifact-md-1".to_string(), "some description".to_string())],
    );
    is_ok!(artifact);
}

#[test]
/// Requirement validation: verifies invalid artifact names are rejected.
fn new_artifact_invalid_name_error() {
    let artifact = Artifact::new(
        "bad artifact name",
        "artifact description",
        &[("artifact-md-1".to_string(), "some description".to_string())],
    );

    let error = is_error!(artifact);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .starts_with("There is an issue with your artifact name 'bad artifact name': ")
    );
}

#[test]
/// Requirement validation: verifies invalid artifact descriptions are rejected.
fn new_artifact_invalid_description_error() {
    let artifact = Artifact::new(
        "artifact-1",
        "",
        &[("artifact-md-1".to_string(), "some description".to_string())],
    );
    let error = is_error!(artifact);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .starts_with("There is an issue with your artifact description '': ")
    );
}

#[test]
/// Requirement validation: verifies invalid artifact metadata keys are rejected.
fn new_artifact_invalid_metadata_error() {
    let artifact = Artifact::new(
        "artifact-1",
        "This is an artifact",
        &[("bad key name".to_string(), "some description".to_string())],
    );
    let error = is_error!(artifact);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .starts_with("There is an issue with your artifact metadata 'bad key name': ")
    );
}
