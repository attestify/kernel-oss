//! Tests for assurance-report `Action`, covering the use/str builders and validation behavior.
//!
//! Bounded unit under test: `assurance_report::Action`.
//! Public interfaces verified: `builder`, `ActionBuilder::default`, `use_*` methods, and string
//! builder methods.
//! Logical paths covered: valid construction, override precedence, and invalid inputs across name,
//! outcome, reason, and file path/signature fields.
//! Requirement validation points: standards-aligned action behavior for assurance reports.

use crate::algorithms::signature_algorithm::Signature;
use crate::algorithms::signature_algorithm::SignatureType::SHA256;
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::action::{Action, ActionBuilder};
use crate::values::specification::assurance_report::signed_file::SignedFile;
use crate::values::specification::description::Description;
use crate::values::specification::name::Name;
use crate::values::specification::outcome::Outcome;
use test_framework_oss::{is_ok, kernel_error_starts_with};

fn signature(value: &str) -> Signature {
    is_ok!(Signature::try_new(SHA256, value))
}

fn signed_file(path: &str, sig: &str) -> SignedFile {
    is_ok!(SignedFile::new(path, &signature(sig)))
}

fn action_from_use() -> Action {
    let test_file = signed_file("./some-location/file.txt", "the-test-file-signature");
    let evidence_file = signed_file("./some-location/file.txt", "the-evidence-file-signature");

    is_ok!(
        Action::builder()
            .use_name(&is_ok!(Name::try_from("action-name")))
            .use_outcome(&Outcome::PASS)
            .use_reason(&is_ok!(Description::try_from("action reason")))
            .use_test_file_signature(&test_file)
            .use_evidence_file_signature(&evidence_file)
            .try_build()
    )
}

fn action_from_strs(test_path: &str, evidence_path: &str) -> Action {
    is_ok!(
        ActionBuilder::default()
            .name("action-name")
            .outcome("pass")
            .reason("action reason")
            .test_file_path(test_path)
            .test_file_signature("SHA256[testsignature]")
            .evidence_file_path(evidence_path)
            .evidence_file_signature("SHA256[evidencesignature]")
            .try_build()
    )
}

#[test]
/// Requirement validation: verifies the builder accepts `use_*` inputs only.
fn builder_use_only_success() {
    let action = action_from_use();

    assert_eq!(action.name(), &is_ok!(Name::try_from("action-name")));
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(
        action.reason(),
        &is_ok!(Description::try_from("action reason"))
    );
    assert_eq!(
        action.test_file(),
        &signed_file("./some-location/file.txt", "the-test-file-signature")
    );
    assert_eq!(
        action.evidence_file(),
        &signed_file("./some-location/file.txt", "the-evidence-file-signature")
    );
}

#[test]
/// Requirement validation: verifies the default builder path succeeds.
fn default_builder_success() {
    let action = action_from_strs("./some-test/file.txt", "./some-evidence/file.txt");

    assert_eq!(action.name(), &is_ok!(Name::try_from("action-name")));
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(
        action.reason(),
        &is_ok!(Description::try_from("action reason"))
    );
    assert_eq!(
        action.test_file(),
        &signed_file("./some-test/file.txt", "testsignature")
    );
    assert_eq!(
        action.evidence_file(),
        &signed_file("./some-evidence/file.txt", "evidencesignature")
    );
}

#[test]
/// Requirement validation: verifies the string-only builder path succeeds.
fn builder_str_only_success() {
    let action = action_from_strs("./some-test/file.txt", "./some-evidence/file.txt");

    assert_eq!(action.name(), &is_ok!(Name::try_from("action-name")));
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(
        action.reason(),
        &is_ok!(Description::try_from("action reason"))
    );
    assert_eq!(
        action.test_file(),
        &signed_file("./some-test/file.txt", "testsignature")
    );
    assert_eq!(
        action.evidence_file(),
        &signed_file("./some-evidence/file.txt", "evidencesignature")
    );
}

#[test]
/// Requirement validation: verifies `use_*` inputs override later string inputs.
fn builder_ensure_use_overrides_str_success() {
    let test_file = signed_file("./some-location/file.txt", "the-test-file-signature");
    let evidence_file = signed_file("./some-location/file.txt", "the-evidence-file-signature");

    let action = is_ok!(
        Action::builder()
            .use_name(&is_ok!(Name::try_from("action-name")))
            .use_outcome(&Outcome::PASS)
            .use_reason(&is_ok!(Description::try_from("action reason")))
            .use_test_file_signature(&test_file)
            .use_evidence_file_signature(&evidence_file)
            .name("name-shoud-not-appear")
            .outcome("fail")
            .reason("bad reason")
            .test_file_path("./some-bad-test/file.txt")
            .test_file_signature("SHA256[testsignature]")
            .evidence_file_path("./some-bad-evidence/file.txt")
            .evidence_file_signature("SHA256[evidencesignature]")
            .try_build()
    );

    assert_eq!(action.name(), &is_ok!(Name::try_from("action-name")));
    assert_eq!(action.outcome(), &Outcome::PASS);
    assert_eq!(
        action.reason(),
        &is_ok!(Description::try_from("action reason"))
    );
    assert_eq!(action.test_file(), &test_file);
    assert_eq!(action.evidence_file(), &evidence_file);
}

#[test]
/// Requirement validation: verifies invalid names are rejected.
fn builder_invalid_name_error() {
    let test_file = signed_file("./some-location/file.txt", "the-test-file-signature");
    let evidence_file = signed_file("./some-location/file.txt", "the-evidence-file-signature");

    let result = Action::builder()
        .name("action name")
        .use_outcome(&Outcome::PASS)
        .reason("action reason")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the name 'action name'. "
    );
}

#[test]
/// Requirement validation: verifies invalid outcomes are rejected.
fn builder_invalid_outcome_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("not-an-outcome")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the outcome 'not-an-outcome'. "
    );
}

#[test]
/// Requirement validation: verifies invalid reasons are rejected.
fn builder_invalid_reason_error() {
    let test_file = signed_file("./some-location/file.txt", "the-test-file-signature");
    let evidence_file = signed_file("./some-location/file.txt", "the-evidence-file-signature");

    let result = Action::builder()
        .name("action-name")
        .use_outcome(&Outcome::PASS)
        .reason("  ")
        .use_test_file_signature(&test_file)
        .use_evidence_file_signature(&evidence_file)
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the reason '  '. "
    );
}

#[test]
/// Requirement validation: verifies invalid test file paths are rejected.
fn builder_invalid_test_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the test file path ''. "
    );
}

#[test]
/// Requirement validation: verifies invalid test file signatures are rejected.
fn builder_invalid_test_file_signature_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("BILL[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the test file signature 'BILL[testsignature]'. "
    );
}

#[test]
/// Requirement validation: verifies invalid evidence file paths are rejected.
fn builder_invalid_evidence_file_path_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("")
        .evidence_file_signature("SHA256[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the evidence file path ''. "
    );
}

#[test]
/// Requirement validation: verifies invalid evidence file signatures are rejected.
fn builder_invalid_evidence_file_signature_error() {
    let result = Action::builder()
        .name("action-name")
        .outcome("pass")
        .reason("action reason")
        .test_file_path("./some-test/file.txt")
        .test_file_signature("SHA256[testsignature]")
        .evidence_file_path("./some-evidence/file.txt")
        .evidence_file_signature("BILL[evidencesignature]")
        .try_build();

    kernel_error_starts_with!(
        result,
        Kind::InvalidInput,
        Audience::User,
        "There is an issue with the evidence file signature 'BILL[evidencesignature]'. "
    );
}
