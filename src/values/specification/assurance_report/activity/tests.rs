//! Tests for assurance-report `Activity`, covering builder overrides and action attachment.
//!
//! Bounded unit under test: `assurance_report::Activity`.
//! Public interfaces verified: `builder`, `Activity::builder`, `Builder::default`, and the action
//! attachment helpers.
//! Logical paths covered: valid construction, override behavior, multiple action append flows, and
//! invalid name handling.
//! Requirement validation points: standards-aligned activity behavior for assurance reports.

use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activity::{Activity, Builder};
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::name::Name;
use crate::values::specification::outcome::Outcome;
use test_framework_oss::{is_ok, kernel_error_starts_with};

fn test_file_signature() -> Signature {
    is_ok!(Signature::try_new(SHA256, "the-test-file-signature"))
}

fn evidence_file_signature() -> Signature {
    is_ok!(Signature::try_new(SHA256, "the-evidence-file-signature"))
}

fn test_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &test_file_signature()
    ))
}

fn evidence_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &evidence_file_signature()
    ))
}

fn testing_action() -> Action {
    is_ok!(
        Action::builder()
            .name("action-name")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    )
}

#[test]
/// Requirement validation: verifies the activity builder creates a valid activity.
fn builder_success() {
    let activity = Activity::builder().name("activity-name").try_build();
    let activity = is_ok!(activity);

    assert_eq!(activity.name(), &is_ok!(Name::try_from("activity-name")));
    assert_eq!(activity.count(), 0);
}

#[test]
/// Requirement validation: verifies the default builder creates a valid activity.
fn default_builder_success() {
    let activity = Builder::default().name("activity-name").try_build();
    let activity = is_ok!(activity);

    assert_eq!(activity.name(), &is_ok!(Name::try_from("activity-name")));
    assert_eq!(activity.count(), 0);
}

#[test]
/// Requirement validation: verifies `use_name` overrides later `name` inputs.
fn builder_use_override_success() {
    let activity = Activity::builder()
        .use_name(&is_ok!(Name::try_from("override-name")))
        .name("activity-name")
        .try_build();
    let activity = is_ok!(activity);

    assert_eq!(activity.name(), &is_ok!(Name::try_from("override-name")));
    assert_eq!(activity.count(), 0);
}

#[test]
/// Requirement validation: verifies a single action can be added to an activity.
fn add_action_success() {
    let action = testing_action();

    let activity = Activity::builder()
        .name("activity-name")
        .append_action(&action)
        .try_build();
    let activity = is_ok!(activity);

    assert_eq!(activity.actions(), &vec![action]);
    assert_eq!(activity.count(), 1);
}

#[test]
/// Requirement validation: verifies multiple actions can be added to an activity.
fn add_many_actions_success() {
    let action_1 = testing_action();
    let action_2 = is_ok!(
        Action::builder()
            .name("action-2")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_3 = is_ok!(
        Action::builder()
            .name("action-3")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_4 = is_ok!(
        Action::builder()
            .name("action-4")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_5 = is_ok!(
        Action::builder()
            .name("action-5")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_6 = is_ok!(
        Action::builder()
            .name("action-6")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_7 = is_ok!(
        Action::builder()
            .name("action-7")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_8 = is_ok!(
        Action::builder()
            .name("action-8")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_9 = is_ok!(
        Action::builder()
            .name("action-9")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action_10 = is_ok!(
        Action::builder()
            .name("action-10")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );

    let activity = Activity::builder()
        .name("activity-name")
        .append_action(&action_1)
        .append_action(&action_2)
        .append_action(&action_3)
        .append_action(&action_4)
        .append_action(&action_5)
        .append_action(&action_6)
        .append_action(&action_7)
        .append_action(&action_8)
        .append_action(&action_9)
        .append_action(&action_10)
        .try_build();
    let activity = is_ok!(activity);

    assert_eq!(
        activity.actions(),
        &vec![
            action_1, action_2, action_3, action_4, action_5, action_6, action_7, action_8,
            action_9, action_10
        ]
    );
    assert_eq!(activity.count(), 10);
}

#[test]
/// Requirement validation: verifies missing names are rejected.
fn invalid_no_name_error() {
    let result = Activity::builder().try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "Please provide an Activity name."
    );
}

#[test]
/// Requirement validation: verifies invalid activity names are rejected.
fn invalid_name_error() {
    let result = Activity::builder().name("invalid name").try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The activity name 'invalid name' is invalid. "
    );
}

#[test]
/// Requirement validation: verifies duplicate actions are rejected.
fn duplicate_action_error() {
    let action = is_ok!(
        Action::builder()
            .name("action-name")
            .outcome("pass")
            .reason("reason")
            .test_file_path("test_file.txt")
            .test_file_signature("SHA256[thesig]")
            .evidence_file_path("evidence_file.txt")
            .evidence_file_signature("SHA256[thesig]")
            .try_build()
    );
    let dup_action = is_ok!(
        Action::builder()
            .name("action-name")
            .outcome("pass")
            .reason("reason duplicative")
            .test_file_path("test_file.txt")
            .test_file_signature("SHA256[thesig]")
            .evidence_file_path("evidence_file.txt")
            .evidence_file_signature("SHA256[thesig]")
            .try_build()
    );

    let result = Activity::builder()
        .name("activity-name")
        .append_action(&action)
        .append_action(&dup_action)
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "The activity 'activity-name' has one or more actions with the same action name of 'action-name'. All action names for a given activity must be unique. Review the actions for this activity to ensure each action has a unique name."
    );
}
