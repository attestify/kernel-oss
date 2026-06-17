//! Tests for assurance-procedure `Activity`, covering action attachment and evidence tracking.
//!
//! Bounded unit under test: `assurance_procedure::Activity`.
//! Public interfaces verified: `new`, `append_action`, and `add_expected_evidence`.
//! Logical paths covered: successful construction, action addition, expected-evidence insertion,
//! and validation failures for invalid name/description/evidence inputs.
//! Requirement validation points: standards-aligned activity behavior for assurance procedures.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::assurance_procedure::activity::Activity;
use test_framework_oss::{is_error, is_ok};

/** Happy path tests **/

#[test]
/// Requirement validation: verifies `Activity::new` constructs a valid activity.
fn new_success() {
    let procedure = is_ok!(Activity::new("procedure-1", "Short Desc", "Long Desc"));
    assert_eq!(procedure.name.value, "procedure-1");
    assert_eq!(procedure.short.value, "Short Desc");
    assert_eq!(procedure.description.value, "Long Desc");
    assert_eq!(procedure.expected_evidence.len(), 0);
    assert_eq!(procedure.action_count(), 0);
}

#[test]
/// Requirement validation: verifies actions can be appended to an activity.
fn add_activity_success() {
    let activity = is_ok!(Activity::new("procedure-1", "Short Desc", "Long Desc"));
    let action = is_ok!(
        Action::builder()
            .name("action-1")
            .short_description("Short Desc")
            .long_description("Long Desc")
            .test_file_path("test_file_path")
            .evidence_file_path("evidence_file_path")
            .try_build()
    );

    let updated_activity = activity.append_action(action);

    assert_eq!(updated_activity.action_count(), 1);
    assert_eq!(updated_activity.actions[0].name.value, "action-1");
    assert_eq!(updated_activity.actions[0].short.value, "Short Desc");
    assert_eq!(updated_activity.actions[0].description.value, "Long Desc");
    assert_eq!(updated_activity.actions[0].test.as_str(), "test_file_path");
    assert_eq!(
        updated_activity.actions[0].evidence.as_str(),
        "evidence_file_path"
    );
}

#[test]
/// Requirement validation: verifies expected evidence can be added to an activity.
fn activity_add_expected_evidence_success() {
    let activity = is_ok!(Activity::new("activity-1", "Short Desc", "Long Desc"));
    let activity = is_ok!(activity.add_expected_evidence("file_path"));
    assert_eq!(activity.expected_evidence.len(), 1);
    assert_eq!(activity.expected_evidence[0].as_str(), "file_path");
    assert_eq!(activity.action_count(), 0);
}

/** Sad path tests **/

#[test]
/// Requirement validation: verifies invalid activity names are rejected.
fn new_activity_invalid_name_error() {
    let err = is_error!(Activity::new("", "Short Desc", "Long Desc"));
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(
        err.message.starts_with(
            "There is an issue with the activity information: The name has an issue: "
        )
    );
}

#[test]
/// Requirement validation: verifies invalid short descriptions are rejected.
fn new_activity_invalid_short_description_error() {
    let err = is_error!(Activity::new("activity-1", "", "Long Desc"));
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with(
        "There is an issue with the activity information: The short description has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies invalid long descriptions are rejected.
fn new_activity_long_description_error() {
    let err = is_error!(Activity::new("activity-1", "Short Desc", ""));
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with(
        "There is an issue with the activity information: The long description has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies invalid expected evidence entries are rejected.
fn activity_add_expected_evidence_invalid_error() {
    let activity = is_ok!(Activity::new("activity-1", "Short Desc", "Long Desc"));
    let err = is_error!(activity.add_expected_evidence(""));
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with(
        "There is an issue with the activity information: There was an issue adding the expected evidence '': "
    ));
}
