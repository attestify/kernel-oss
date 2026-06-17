//! Tests for assurance-procedure `Artifacts`, covering merge and duplicate detection behavior.
//!
//! Bounded unit under test: `assurance_procedure::Artifacts`.
//! Public interfaces verified: `default`, `add`, and `merge`.
//! Logical paths covered: successful insertion, merging, and duplicate-artifact rejection.
//! Requirement validation points: standards-aligned artifact collection behavior for assurance
//! procedures.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::artifact::Artifact;
use crate::values::specification::assurance_procedure::artifacts::Artifacts;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies artifacts can be added successfully.
fn add_artifact_success() {
    let artifacts = Artifacts::default();
    let updated_artifacts = is_ok!(artifacts.add(
        "artifact-1",
        "some artifact description",
        &[("key-1".to_string(), "some value".to_string())],
    ));
    assert_eq!(updated_artifacts.count(), 1);
}

#[test]
/// Requirement validation: verifies artifacts can be merged successfully.
fn merge_artifact_success() {
    let artifacts = Artifacts::default();
    let artifact = is_ok!(Artifact::new(
        "artifact-1",
        "some description",
        &[("key-1".to_string(), "some value".to_string())],
    ));
    let new_artifacts = is_ok!(artifacts.merge(&artifact));
    assert_eq!(new_artifacts.count(), 1);
}

#[test]
/// Requirement validation: verifies duplicate artifacts are rejected on add.
fn add_artifact_already_exists_error() {
    let artifacts = Artifacts::default();
    let updated_artifacts = is_ok!(artifacts.add(
        "artifact-1",
        "some description",
        &[("key-1".to_string(), "value-1".to_string())],
    ));
    let error = is_error!(updated_artifacts.add(
        "artifact-1",
        "description",
        &[("key".to_string(), "value".to_string())],
    ));

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The artifact  'artifact-1' cannot be added because it already exists.")
    );
}

#[test]
/// Requirement validation: verifies duplicate artifacts are rejected during merge.
fn merge_artifact_already_exists_error() {
    let artifacts = Artifacts::default();
    let first_update = is_ok!(artifacts.add(
        "artifact-1",
        "description",
        &[("key".to_string(), "value".to_string())],
    ));

    let artifact_to_merge = is_ok!(Artifact::new(
        "artifact-1",
        "description",
        &[("key".to_string(), "value".to_string())],
    ));
    let second_update_result = first_update.merge(&artifact_to_merge);

    let error = is_error!(second_update_result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("The artifact 'artifact-1' cannot be added because an artifact with the name 'artifact-1' already exists."));
}
