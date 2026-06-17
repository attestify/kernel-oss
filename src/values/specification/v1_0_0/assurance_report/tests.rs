//! Tests for the v1.0.0 assurance-report wrapper, covering builder success, overrides, and
//! validation failures.
//!
//! Bounded unit under test: the `v1_0_0::AssuranceReport` builder wrapper.
//! Public interfaces verified: `Builder::new`, `Builder::default`, builder setters, and `try_build`.
//! Logical paths covered: valid construction, override precedence, missing required inputs,
//! invalid subject/procedure data, invalid activity data, and invalid additional information.
//! Requirement validation points: standards-aligned compatibility wrapper behavior for assurance
//! reports.

use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values;
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::additional_information::AdditionalInformation;
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::description::Description;
use crate::values::specification::metadata::MetaData;
use crate::values::specification::name::Name;
use crate::values::specification::outcome::Outcome;
use crate::values::specification::procedure::Procedure;
use crate::values::specification::subject::Subject;
use crate::values::specification::traits::AssuranceReport;
use crate::values::specification::v1_0_0::assurance_report::Builder;
use test_framework_oss::{is_ok, kernel_error_starts_with};

fn signature(value: &str) -> Signature {
    is_ok!(Signature::try_new(SHA256, value))
}

fn signed_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &signature("the-signature")
    ))
}

fn test_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &signature("the-test-file-signature")
    ))
}

fn evidence_file() -> SignedFile {
    is_ok!(SignedFile::new(
        "./some-location/file.txt",
        &signature("the-evidence-file-signature")
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

fn activity(name: &str) -> Activity {
    is_ok!(Activity::builder().name(name).try_build())
}

fn report_base() -> Builder {
    Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
}

#[test]
/// Requirement validation: verifies the builder creates a valid assurance report.
fn builder_success() {
    let existing_metadata = vec![
        ("key-1".to_string(), "value 1".to_string()),
        ("key-2".to_string(), "value 2".to_string()),
    ];

    let result = report_base()
        .merge_metadata(&existing_metadata)
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();

    let report = is_ok!(result);
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(
        report.kind(),
        values::specification::kind::Kind::AssuranceReport
    );
    assert_eq!(report.metadata().data.len(), 3);
    assert_eq!(report.metadata().data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(
        report.metadata().data[0].1,
        is_ok!(Description::try_from("value"))
    );
    assert_eq!(report.subject().nrn.value, "nrn:procedure:example");
    assert_eq!(report.subject().id.value, "somesubjectid");
    assert_eq!(report.procedure().repository, "https://some-location.com");
    assert_eq!(report.activities().action_count(), 1);
    assert_eq!(report.additional_info().count(), 1);
    assert_eq!(
        report.additional_info().list()[0].value,
        "additional information"
    );
    assert_eq!(report.summary().action_count, 1);
    assert_eq!(report.summary().actions_run, 1);
    assert_eq!(report.summary().pass, 1);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);
}

#[test]
/// Requirement validation: verifies the default builder creates a valid assurance report.
fn default_builder_success() {
    let result = report_base()
        .add_activity(&activity("activity"))
        .additional_information("additional information")
        .try_build();
    let report = is_ok!(result);

    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(report.summary().outcome, Outcome::PASS);
}

#[test]
/// Requirement validation: verifies the builder handles multiple metadata, activities, actions,
/// and additional-information entries.
fn builder_multiple_metadata_activities_actions_and_additional_information_success() {
    let action1 = testing_action();
    let action2 = is_ok!(
        Action::builder()
            .name("action-name-2")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );
    let action3 = is_ok!(
        Action::builder()
            .name("action-name-3")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file())
            .use_evidence_file_signature(&evidence_file())
            .try_build()
    );

    let result = report_base()
        .add_metadata("key2", "value2")
        .add_activity(&activity("activity"))
        .add_activity(&activity("activity-1"))
        .add_activity(&activity("activity-2"))
        .add_action("activity", &action1)
        .add_action("activity-3", &action2)
        .add_action("activity-4", &action3)
        .additional_information("additional information")
        .additional_information("additional information 2")
        .try_build();

    let report = is_ok!(result);
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(
        report.kind(),
        values::specification::kind::Kind::AssuranceReport
    );
    assert_eq!(report.metadata().data.len(), 2);
    assert_eq!(report.metadata().data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(
        report.metadata().data[0].1,
        is_ok!(Description::try_from("value"))
    );
    assert_eq!(report.metadata().data[1].0, is_ok!(Name::try_from("key2")));
    assert_eq!(
        report.metadata().data[1].1,
        is_ok!(Description::try_from("value2"))
    );
    assert_eq!(report.subject().nrn.value, "nrn:procedure:example");
    assert_eq!(report.subject().id.value, "somesubjectid");
    assert_eq!(report.procedure().repository, "https://some-location.com");
    assert_eq!(report.activities().list().len(), 5);
    assert_eq!(report.activities().list()[0].name.value, "activity");
    assert_eq!(report.activities().list()[1].name.value, "activity-1");
    assert_eq!(report.activities().list()[2].name.value, "activity-2");
    assert_eq!(report.activities().list()[3].name.value, "activity-3");
    assert_eq!(report.activities().list()[4].name.value, "activity-4");
    assert_eq!(report.additional_info().count(), 2);
    assert_eq!(
        report.additional_info().list()[0].value,
        "additional information"
    );
    assert_eq!(
        report.additional_info().list()[1].value,
        "additional information 2"
    );
    assert_eq!(report.summary().action_count, 3);
    assert_eq!(report.summary().actions_run, 3);
    assert_eq!(report.summary().pass, 3);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);
}

#[test]
/// Requirement validation: verifies explicit `use_*` setters override earlier inputs.
fn use_functions_override_success() {
    let existing_metadata = vec![("key-1".to_string(), "value 1".to_string())];

    let mut overriding_metadata = MetaData::default();
    is_ok!(overriding_metadata.add("override-key", "value"));
    is_ok!(overriding_metadata.add("override-key-2", "value 2"));
    is_ok!(overriding_metadata.add("override-key-3", "value 3"));
    is_ok!(overriding_metadata.add("override-key-4", "value 4"));

    let overriding_subject = is_ok!(Subject::try_new(
        "nrn:sourcecode:override/subject",
        "overridesubjectid"
    ));
    let overriding_procedure = is_ok!(Procedure::try_new(
        "https://some-override-location.com",
        "some/override/location"
    ));

    let test_file_sig = signed_file();
    let evidence_file_sig = signed_file();
    let action1 = testing_action();
    let action2 = is_ok!(
        Action::builder()
            .name("action-name-2")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file_sig)
            .use_evidence_file_signature(&evidence_file_sig)
            .try_build()
    );
    let action3 = is_ok!(
        Action::builder()
            .name("action-name-3")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file_sig)
            .use_evidence_file_signature(&evidence_file_sig)
            .try_build()
    );
    let action4 = is_ok!(
        Action::builder()
            .name("action-name-4")
            .use_outcome(&Outcome::PASS)
            .reason("action reason")
            .use_test_file_signature(&test_file_sig)
            .use_evidence_file_signature(&evidence_file_sig)
            .try_build()
    );

    let activity1 = is_ok!(
        Activity::builder()
            .name("activity")
            .append_action(&action1)
            .append_action(&action2)
            .try_build()
    );
    let activity2 = is_ok!(
        Activity::builder()
            .name("activity-2")
            .append_action(&action3)
            .append_action(&action4)
            .try_build()
    );

    let overriding_activities = is_ok!(
        Activities::builder()
            .add_activity(&activity1)
            .add_activity(&activity2)
            .try_build()
    );
    let overriding_additional_info = is_ok!(
        AdditionalInformation::builder()
            .append("override-additional-info")
            .append("override-additional-info-2")
            .try_build()
    );

    let result = report_base()
        .merge_metadata(&existing_metadata)
        .add_activity(&activity("activity-first"))
        .add_action("activity-first", &testing_action())
        .additional_information("additional information first")
        .use_metadata(&overriding_metadata)
        .use_subject(&overriding_subject)
        .use_procedure(&overriding_procedure)
        .use_activities(&overriding_activities)
        .use_additional_information(&overriding_additional_info)
        .try_build();

    let report = is_ok!(result);
    assert_eq!(report.api_version().as_string(), "1.0.0".to_string());
    assert_eq!(
        report.kind(),
        values::specification::kind::Kind::AssuranceReport
    );
    assert_eq!(report.metadata().data.len(), 4);
    assert_eq!(
        report.subject().nrn.value,
        "nrn:sourcecode:override/subject"
    );
    assert_eq!(report.subject().id.value, "overridesubjectid");
    assert_eq!(
        report.procedure().repository,
        "https://some-override-location.com"
    );
    assert_eq!(report.procedure().directory, "some/override/location");
    assert_eq!(report.activities().action_count(), 4);
    assert_eq!(report.additional_info().count(), 2);
    assert_eq!(report.summary().action_count, 4);
    assert_eq!(report.summary().actions_run, 4);
    assert_eq!(report.summary().pass, 4);
    assert_eq!(report.summary().fail, 0);
    assert_eq!(report.summary().inconclusive, 0);
    assert_eq!(report.summary().outcome, Outcome::PASS);
}

#[test]
/// Requirement validation: verifies a report can be built without any action or activity calls.
fn builder_no_action_or_activity_call_success() {
    let result = report_base()
        .additional_information("additional information")
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.activities().list().len(), 0);
}

#[test]
/// Requirement validation: verifies a report can be built without an explicit activity call.
fn builder_no_activity_call_success() {
    let result = report_base()
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.activities().list().len(), 1);
}

#[test]
/// Requirement validation: verifies a report can be built without an explicit action call.
fn builder_no_action_call_success() {
    let result = report_base()
        .add_activity(&activity("activity"))
        .additional_information("additional information")
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.activities().list().len(), 1);
}

#[test]
/// Requirement validation: verifies a report can be built without additional information.
fn builder_no_additional_information_success() {
    let result = report_base()
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.additional_info().count(), 0);
}

#[test]
/// Requirement validation: verifies metadata can be upserted during report construction.
fn builder_upsert_metadata_success() {
    let result = report_base()
        .add_metadata("key-2", "value 2")
        .upsert_metadata("key", "updated_value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();
    let report = is_ok!(result);

    assert_eq!(report.metadata().data.len(), 2);
    assert_eq!(
        report.metadata().get("key"),
        Some("updated_value".to_string())
    );
    assert_eq!(report.metadata().get("key-2"), Some("value 2".to_string()));
}

#[test]
/// Requirement validation: verifies metadata upsert inserts a new key when it is missing.
fn builder_upsert_metadata_new_key_success() {
    let result = report_base()
        .add_metadata("key-2", "value 2")
        .upsert_metadata("missing-key", "inserted-value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();
    let report = is_ok!(result);

    assert_eq!(report.metadata().data.len(), 3);
    assert_eq!(
        report.metadata().get("missing-key"),
        Some("inserted-value".to_string())
    );
}

#[test]
/// Requirement validation: verifies a report can be built with no metadata.
fn builder_no_metadata_success() {
    let result = Builder::new()
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.metadata().data.len(), 0);
}

#[test]
/// Requirement validation: verifies invalid metadata is rejected.
fn builder_bad_metadata_error() {
    let report_result = Builder::new()
        .add_metadata("a bad key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. The Metadata has an issue."
    );
}

#[test]
/// Requirement validation: verifies a missing subject NRN is rejected.
fn builder_no_subject_nrn_error() {
    let report_result = Builder::new()
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. The subject NRN is required, although it was not provided."
    );
}

#[test]
/// Requirement validation: verifies invalid subject NRNs are rejected.
fn builder_bad_subject_nrn_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("some bad subject nrn")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue with your Subject data. "
    );
}

#[test]
/// Requirement validation: verifies a missing subject ID is rejected.
fn builder_no_subject_id_error() {
    let report_result = Builder::new()
        .subject_nrn("nrn:procedure:example")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. The subject ID is required, although it was not provided."
    );
}

#[test]
/// Requirement validation: verifies invalid subject IDs are rejected.
fn builder_bad_subject_id_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue with your Subject data. "
    );
}

#[test]
/// Requirement validation: verifies a missing procedure repository is rejected.
fn builder_no_procedure_repository_error() {
    let report_result = Builder::new()
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_directory("some/location")
        .add_metadata("key", "value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. The procedure repository link is required, but was not provided."
    );
}

#[test]
/// Requirement validation: verifies invalid procedure repositories are rejected.
fn builder_bad_procedure_repository_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("bad procedure repo")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue with your procedure data. "
    );
}

#[test]
/// Requirement validation: verifies a missing procedure directory is rejected.
fn builder_no_procedure_directory_error() {
    let report_result = Builder::new()
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .add_metadata("key", "value")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .additional_information("additional information")
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. The procedure directory is required, but was not provided."
    );
}

#[test]
/// Requirement validation: verifies invalid procedure directories are rejected.
fn builder_bad_procedure_directory_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://example.com")
        .procedure_directory("/some/bad/location-/")
        .add_activity(&activity("activity"))
        .add_action("procedure", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue with your procedure data. "
    );
}

#[test]
/// Requirement validation: verifies invalid activity names are rejected when adding actions.
fn builder_bad_activity_name_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("good-procedure-name"))
        .add_action("bad- activity name", &testing_action())
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue adding your Activity. "
    );
}

#[test]
/// Requirement validation: verifies empty additional-information entries are rejected.
fn builder_additional_information_empty_string_error() {
    let report_result = Builder::new()
        .add_metadata("key", "value")
        .subject_nrn("nrn:procedure:example")
        .subject_id("somesubjectid")
        .procedure_repository("https://some-location.com")
        .procedure_directory("some/location")
        .add_activity(&activity("activity"))
        .add_action("action", &testing_action())
        .additional_information("")
        .try_build();

    kernel_error_starts_with!(
        report_result,
        Kind::InvalidInput,
        Audience::User,
        "The AssuranceReport could not be created. There is an issue adding your Additional Information to the report. "
    );
}
