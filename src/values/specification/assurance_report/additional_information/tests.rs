//! Tests for assurance-report `AdditionalInformation`, covering append and deduplication.
//!
//! Bounded unit under test: `assurance_report::AdditionalInformation`.
//! Public interfaces verified: `builder`, `AdditionalInformationBuilder::default`, and `try_build`.
//! Logical paths covered: successful append, duplicate suppression, and invalid entry rejection.
//! Requirement validation points: standards-aligned additional-information behavior for reports.

use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::additional_information::{
    AdditionalInformation, AdditionalInformationBuilder,
};
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_contains;

#[test]
/// Requirement validation: verifies additional information can be appended successfully.
fn add_success() {
    let result = AdditionalInformation::builder()
        .append("This is a test")
        .try_build();

    let additional_info = is_ok!(result);
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");
}

#[test]
/// Requirement validation: verifies the default builder can append information successfully.
fn default_builder_success() {
    let result = AdditionalInformationBuilder::default()
        .append("This is a test")
        .try_build();

    let additional_info = is_ok!(result);
    assert_eq!(additional_info.count(), 1);
}

#[test]
/// Requirement validation: verifies duplicate entries are suppressed.
fn add_duplicate_success() {
    let result = AdditionalInformation::builder()
        .append("This is a test")
        .append("This is a test")
        .try_build();

    let additional_info = is_ok!(result);
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");
}

#[test]
/// Requirement validation: verifies invalid additional-information entries are rejected.
fn add_error() {
    let result = AdditionalInformation::builder().append(" ").try_build();

    kernel_error_contains!(
        &result,
        Kind::InvalidInput,
        Audience::User,
        "We could not add the additional information ' '. "
    );
}
