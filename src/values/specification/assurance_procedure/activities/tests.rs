//! Tests for assurance-procedure `Activities`, covering merge and action-routing behavior.
//!
//! Bounded unit under test: `assurance_procedure::Activities`.
//! Public interfaces verified: `default`, `add`, `merge`, and `add_activity`.
//! Logical paths covered: empty state, merging, duplicate suppression, action routing, and
//! invalid activity names.
//! Requirement validation points: standards-aligned activity collection behavior for assurance
//! procedures.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::assurance_procedure::activities::Activities;
use crate::values::specification::assurance_procedure::activity::Activity;
use test_framework_oss::{is_error, is_ok};

fn action(name: &str, short: &str, long: &str) -> Action {
    is_ok!(
        Action::builder()
            .name(name)
            .short_description(short)
            .long_description(long)
            .test_file_path("test_file")
            .evidence_file_path("evidence")
            .try_build()
    )
}

#[test]
/// Requirement validation: verifies the default activities collection starts empty.
fn default_activities_success() {
    let activities = Activities::default();
    assert_eq!(activities.count(), 0);
    assert_eq!(activities.action_count(), 0);
}

#[test]
/// Requirement validation: verifies a new activity can be added to the collection.
fn add_activity_success() {
    let activities = Activities { list: Vec::new() };
    let new_activities = is_ok!(activities.add("activity-1", "Short Desc", "Long Desc"));

    assert_eq!(new_activities.count(), 1);
    assert_eq!(new_activities.list[0].name.value, "activity-1");
    assert_eq!(new_activities.action_count(), 0);
}

#[test]
/// Requirement validation: verifies an activity can be merged into the collection.
fn merge_activity_success() {
    let activities = Activities { list: Vec::new() };
    let activity = is_ok!(Activity::new("activity-1", "Short Desc", "Long Desc"));
    let new_activities = activities.merge(&activity);

    assert_eq!(new_activities.count(), 1);
    assert_eq!(new_activities.list[0].name.value, "activity-1");
    assert_eq!(new_activities.action_count(), 0);
}

#[test]
/// Requirement validation: verifies actions can be added to an existing activity.
fn add_activity_when_activity_exists_success() {
    let activities = Activities::default();
    let activity_one = is_ok!(Activity::new("activity-1", "Short Desc", "Long Desc"));
    let activities = activities.merge(&activity_one);
    let action = action("action-1", "Short Desc", "Long Desc");
    let activities = is_ok!(activities.add_activity("activity-1", &action));

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 1);
}

#[test]
/// Requirement validation: verifies duplicate activities merge without duplication.
fn merge_duplicate_activities_without_duplication_success() {
    let activities = Activities::default();

    let action_1 = action("action-1", "Short Desc", "Long Desc");
    let action_2 = action("action-2", "Short Desc", "Long Desc");
    let copy_action_1 = action("action-1", "Action 1 Short Desc", "Action 1 Long Desc");
    let copy_action_2 = action("action-2", "Action 2 Short Desc", "Action 2 Long Desc");

    let activity_one = is_ok!(Activity::new("action-1", "Short Desc", "Long Desc"))
        .append_action(action_1)
        .append_action(action_2);
    let activity_one_copy = is_ok!(Activity::new("action-1", "Short Desc", "Long Desc"))
        .append_action(copy_action_1)
        .append_action(copy_action_2);

    let activities = activities.merge(&activity_one).merge(&activity_one_copy);

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 2);
}

#[test]
/// Requirement validation: verifies duplicate actions on a named activity do not duplicate entries.
fn add_duplicate_activity_to_existing_activity_without_duplication_success() {
    let activities = Activities::default();
    let activity_one = is_ok!(Activity::new("activity-1", "Short Desc", "Long Desc"));
    let activities = activities.merge(&activity_one);

    let action_1 = action("action-1", "Short Desc", "Long Desc");
    let activities = is_ok!(activities.add_activity("activity-1", &action_1));

    let copy_action_1 = action("action-1", "Short Desc", "Long Desc");
    let activities = is_ok!(activities.add_activity("activity-1", &copy_action_1));

    assert_eq!(activities.count(), 1);
    assert_eq!(activities.action_count(), 1);
}

#[test]
/// Requirement validation: verifies invalid activity names are rejected.
fn add_activity_error() {
    let activities = Activities::default();
    let result = activities.add("", "Short Desc", "Long Desc");

    let err = is_error!(result);
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(
        err.message
            .starts_with("We could not add the activity '' to the list of activities: ")
    );
}

#[test]
/// Requirement validation: verifies actions cannot be added when the activity is missing.
fn add_action_without_existing_activity_error() {
    let activities = Activities::default();
    let action_1 = action("action-1", "Short Desc", "Long Desc");
    let activities = activities.add_activity("activity-1", &action_1);

    let err = is_error!(activities);
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert_eq!(
        err.message,
        "Activity 'activity-1' does not exist. The activity must exist before you can add an action to it.  Please add a activity with the name you provided, a short description, and a long description."
    );
}
