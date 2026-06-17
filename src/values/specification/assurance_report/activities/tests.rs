//! Tests for assurance-report `Activities`, covering collection merge and action routing.
//!
//! Bounded unit under test: `assurance_report::Activities`.
//! Public interfaces verified: `builder`, `Builder::default`, `add_activity`, `add_action`, and
//! `try_build`.
//! Logical paths covered: empty collection, merging, duplicate activity handling, action routing,
//! and invalid activity-name rejection.
//! Requirement validation points: standards-aligned activities collection behavior for reports.

use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activities::{Activities, Builder};
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::name::Name;
use crate::values::specification::outcome::Outcome;
use test_framework_oss::{is_ok, kernel_error_starts_with};

fn file_signature(prefix: &str) -> Signature {
    is_ok!(Signature::try_new(SHA256, prefix))
}

fn signed_file(path: &str, prefix: &str) -> SignedFile {
    is_ok!(SignedFile::new(path, &file_signature(prefix)))
}

fn action(name: &str, test_path: &str, evidence_path: &str) -> Action {
    is_ok!(
        Action::builder()
            .name(name)
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&signed_file(test_path, "the-test-file-signature"))
            .use_evidence_file_signature(&signed_file(evidence_path, "the-evidence-file-signature"))
            .try_build()
    )
}

fn activity(name: &str, actions: &[Action]) -> Activity {
    let mut builder = Activity::builder();
    let mut builder = builder.name(name);
    for action in actions {
        builder = builder.append_action(action);
    }
    is_ok!(builder.try_build())
}

#[test]
/// Requirement validation: verifies the activities builder creates an empty collection.
fn success() {
    let activities = Activities::builder().try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 0);
    assert_eq!(activities.action_count(), 0);
}

#[test]
/// Requirement validation: verifies the default builder creates an empty collection.
fn default_builder_success() {
    let activities = Builder::default().try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 0);
    assert_eq!(activities.action_count(), 0);
}

#[test]
/// Requirement validation: verifies a single activity can be added successfully.
fn add_single_activity_success() {
    let action1 = action(
        "action-1",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action2 = action(
        "action-2",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let activity = activity("some-activity", &[action1.clone(), action2.clone()]);

    let activities = Activities::builder().add_activity(&activity).try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.list()[0], activity);
    assert_eq!(
        activities.list()[0].name,
        is_ok!(Name::try_from("some-activity"))
    );
    assert_eq!(activities.action_count(), 2);
}

#[test]
/// Requirement validation: verifies duplicate activity names merge successfully.
fn add_activity_with_existing_activity_name_success() {
    let action1 = action(
        "action-1",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action2 = action(
        "action-2",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action3 = action(
        "action-3",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action4 = action(
        "action-4",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );

    let activity1 = activity("activity-1", &[action1, action2]);
    let activity2 = activity("activity-1", &[action3, action4]);
    let activities = Activities::builder()
        .add_activity(&activity1)
        .add_activity(&activity2)
        .try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 1);
    assert_eq!(activities.action_count(), 4);
}

#[test]
/// Requirement validation: verifies conflicting duplicate actions are rejected.
fn add_activity_with_same_name_actions_error() {
    let action1 = action(
        "action-1",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action2 = action(
        "action-2",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );
    let action3 = action(
        "action-3",
        "./some-location/file.txt",
        "./some-location/file.txt",
    );

    let activity1 = activity("activity-1", &[action1.clone(), action2]);
    let activity2 = activity("activity-1", &[action3, action1]);

    let result = Activities::builder()
        .add_activity(&activity1)
        .add_activity(&activity2)
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Activities list could not be created. "
    );
}

#[test]
/// Requirement validation: verifies actions can be added to an empty collection.
fn add_action_to_empty_activities_success() {
    let action_1 = action(
        "action-1",
        "./some-location/file-1.txt",
        "./some-location/file-1.txt",
    );

    let activities = Activities::builder()
        .add_action("activity-1", &action_1)
        .try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 1);
    assert_eq!(
        activities.list()[0].name,
        is_ok!(Name::try_from("activity-1"))
    );
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[0].actions[0], action_1);
}

#[test]
/// Requirement validation: verifies actions can be added to an existing activity.
fn add_action_to_existing_activities_success() {
    let action_1 = action(
        "action-1",
        "./some-location/file-1.txt",
        "./some-location/file-1.txt",
    );
    let activity1 = is_ok!(Activity::builder().name("activity-1").try_build());

    let activities = Activities::builder()
        .add_activity(&activity1)
        .add_action("activity-1", &action_1)
        .try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 1);
    assert_eq!(
        activities.list()[0].name,
        is_ok!(Name::try_from("activity-1"))
    );
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[0].actions[0], action_1);
}

#[test]
/// Requirement validation: verifies actions can create a new activity when needed.
fn add_action_with_new_activity_success() {
    let action_1 = action(
        "action-1",
        "./some-location/file-1.txt",
        "./some-location/file-1.txt",
    );
    let activity1 = is_ok!(Activity::builder().name("activity-1").try_build());

    let activities = Activities::builder()
        .add_activity(&activity1)
        .add_action("activity-2", &action_1)
        .try_build();
    let activities = is_ok!(activities);

    assert_eq!(activities.list().len(), 2);
    assert_eq!(
        activities.list()[1].name,
        is_ok!(Name::try_from("activity-2"))
    );
    assert_eq!(activities.action_count(), 1);
    assert_eq!(activities.list()[1].actions[0], action_1);
}

#[test]
/// Requirement validation: verifies invalid activity names are rejected during build.
fn add_action_error_invalid_activity_name() {
    let action_1 = action(
        "action-1",
        "./some-location/file-1.txt",
        "./some-location/file-1.txt",
    );

    let result = Activities::builder()
        .add_action("invalid activity name", &action_1)
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The Activities list could not be created. "
    );
}
