//! Tests for assurance-report `Summary`, covering aggregation logic and default behavior.
//!
//! Bounded unit under test: `assurance_report::Summary`.
//! Public interfaces verified: `default` and `Summary::of`.
//! Public interfaces verified: `Summary::activity_count`, `Summary::action_count`,
//! `Summary::actions_run`, `Summary::pass`, `Summary::fail`, `Summary::inconclusive`,
//! and `Summary::outcome`.
//! Logical paths covered: empty collections, all-pass outcomes, fail precedence, inconclusive
//! precedence, and error handling in summary aggregation.
//! Requirement validation points: standards-aligned report-summary aggregation behavior.

use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::assurance_report::summary::Summary;
use crate::values::specification::outcome::Outcome;
use test_framework_oss::is_ok;

fn test_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &is_ok!(Signature::try_new(SHA256, "the-test-file-signature"))
    ))
}

fn evidence_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &is_ok!(Signature::try_new(SHA256, "the-evidence-file-signature"))
    ))
}

fn action(name: &str, outcome: Outcome) -> Action {
    is_ok!(
        Action::builder()
            .name(name)
            .use_outcome(&outcome)
            .reason("some reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
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

fn activities_for(actions: &[Action]) -> Activities {
    let activity = activity("Test-Activity", actions);
    is_ok!(Activities::builder().add_activity(&activity).try_build())
}

#[test]
/// Requirement validation: verifies the default summary starts empty and inconclusive.
fn default_summary() {
    let summary = Summary::default();
    assert_eq!(summary.activity_count, 0);
    assert_eq!(summary.action_count, 0);
    assert_eq!(summary.actions_run, 0);
    assert_eq!(summary.pass, 0);
    assert_eq!(summary.fail, 0);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}

#[test]
/// Requirement validation: verifies the summary accessors expose the stored values.
fn summary_accessors_success() {
    let summary = Summary {
        activity_count: 2,
        action_count: 4,
        actions_run: 3,
        pass: 1,
        fail: 1,
        inconclusive: 1,
        outcome: Outcome::INCONCLUSIVE,
    };

    assert_eq!(summary.activity_count(), 2);
    assert_eq!(summary.action_count(), 4);
    assert_eq!(summary.actions_run(), 3);
    assert_eq!(summary.pass(), 1);
    assert_eq!(summary.fail(), 1);
    assert_eq!(summary.inconclusive(), 1);
    assert_eq!(summary.outcome(), &Outcome::INCONCLUSIVE);
}

#[test]
/// Requirement validation: verifies a fully passing activity set yields a pass summary.
fn summary_of_success_outcome_pass() {
    let activities = activities_for(&[
        action("Test-Action-1", Outcome::PASS),
        action("Test-Action-2", Outcome::PASS),
        action("Test-Action-3", Outcome::PASS),
        action("Test-Action-4", Outcome::PASS),
    ]);

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 4);
    assert_eq!(summary.fail, 0);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::PASS);
}

#[test]
/// Requirement validation: verifies a fail in the set yields a fail summary.
fn summary_of_success_outcome_inconclusive_with_at_least_one_fail() {
    let activities = activities_for(&[
        action("Test-Action-1", Outcome::PASS),
        action("Test-Action-2", Outcome::PASS),
        action("Test-Action-3", Outcome::PASS),
        action("Test-Action-4", Outcome::FAIL),
    ]);

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 3);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 0);
    assert_eq!(summary.outcome, Outcome::FAIL);
}

#[test]
/// Requirement validation: verifies an inconclusive result is preserved when present.
fn summary_of_success_outcome_inconclusive_with_at_least_one_inconclusive() {
    let activities = activities_for(&[
        action("Test-Action-1", Outcome::PASS),
        action("Test-Action-2", Outcome::PASS),
        action("Test-Action-3", Outcome::FAIL),
        action("Test-Action-4", Outcome::INCONCLUSIVE),
    ]);

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 4);
    assert_eq!(summary.pass, 2);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 1);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}

#[test]
/// Requirement validation: verifies errors keep the summary inconclusive and exclude errored
/// actions from the run count.
fn summary_of_success_outcome_inconclusive_with_at_least_one_error() {
    let activities = activities_for(&[
        action("Test-Action-1", Outcome::PASS),
        action("Test-Action-2", Outcome::FAIL),
        action("Test-Action-3", Outcome::INCONCLUSIVE),
        action("Test-Action-4", Outcome::ERROR),
    ]);

    let summary = Summary::of(&activities);

    assert_eq!(summary.activity_count, 1);
    assert_eq!(summary.action_count, 4);
    assert_eq!(summary.actions_run, 3);
    assert_eq!(summary.pass, 1);
    assert_eq!(summary.fail, 1);
    assert_eq!(summary.inconclusive, 1);
    assert_eq!(summary.outcome, Outcome::INCONCLUSIVE);
}
