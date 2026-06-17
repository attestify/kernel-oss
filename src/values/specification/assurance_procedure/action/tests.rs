//! Tests for assurance-procedure `Action`, covering builder construction and validation.
//!
//! Bounded unit under test: `assurance_procedure::Action`.
//! Public interfaces verified: `builder`, `ActionBuilder::default`, and `try_build`.
//! Logical paths covered: valid construction, missing fields, and invalid field inputs across
//! name, outcome, descriptions, and file-path/signature inputs.
//! Requirement validation points: standards-aligned action builder behavior for assurance
//! procedures.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::action::{Action, ActionBuilder};
use crate::values::specification::description::Description;
use crate::values::specification::file_path::FilePath;
use crate::values::specification::name::Name;
use crate::values::specification::short_description::ShortDescription;
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_starts_with;

#[test]
/// Requirement validation: verifies the action builder creates a valid action.
fn new_success() {
    let result = Action::builder()
        .name("name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    let result = is_ok!(result);
    assert_eq!(result.name, is_ok!(Name::try_from("name")));
    assert_eq!(result.short, is_ok!(ShortDescription::try_from("short")));
    assert_eq!(result.description, is_ok!(Description::try_from("long")));
    assert_eq!(result.test, is_ok!(FilePath::try_from("test")));
    assert_eq!(result.evidence, is_ok!(FilePath::try_from("evidence")));
}

#[test]
/// Requirement validation: verifies the default action builder creates a valid action.
fn default_builder_success() {
    let result = ActionBuilder::default()
        .name("name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    let result = is_ok!(result);
    assert_eq!(result.name, is_ok!(Name::try_from("name")));
}

#[test]
/// Requirement validation: verifies the builder rejects a missing name.
fn no_name_error() {
    let result = Action::builder()
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. The name is required, but was not provided."
    );
}
#[test]
/// Requirement validation: verifies invalid names are rejected.
fn bad_name_error() {
    let result = Action::builder()
        .name("")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. There is an issue with the name ''. "
    );
}

#[test]
/// Requirement validation: verifies the builder rejects a missing short description.
fn no_short_description_error() {
    let result = Action::builder()
        .name("name")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. The short description is required, but was not provided."
    );
}
#[test]
/// Requirement validation: verifies invalid short descriptions are rejected.
fn bad_short_description_error() {
    let result = Action::builder()
        .name("name")
        .short_description("")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. There is an issue with the short description ''. "
    );
}

#[test]
/// Requirement validation: verifies the builder rejects a missing long description.
fn no_long_description_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. The long description is required, but was not provided."
    );
}
#[test]
/// Requirement validation: verifies invalid long descriptions are rejected.
fn bad_long_description_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .long_description("")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. There is an issue with the long description ''. "
    );
}

#[test]
/// Requirement validation: verifies the builder rejects a missing test file path.
fn no_test_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .long_description("long")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. The test file path is required, but was not provided."
    );
}
#[test]
/// Requirement validation: verifies invalid test file paths are rejected.
fn bad_test_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. There is an issue with the test file path ''. "
    );
}

#[test]
/// Requirement validation: verifies the builder rejects a missing evidence file path.
fn no_evidence_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. The evidence file path is required, but was not provided."
    );
}
#[test]
/// Requirement validation: verifies invalid evidence file paths are rejected.
fn bad_evidence_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Action for an Assurance Procedure could not be created. There is an issue with the evidence file path ''. "
    );
}
