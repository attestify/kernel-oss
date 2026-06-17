use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::additional_information::{
    AdditionalInformation, AdditionalInformationBuilder,
};
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_contains;

#[test]
fn add_success() {
    let result = AdditionalInformation::builder()
        .append("This is a test")
        .try_build();

    is_ok!(&result);

    let additional_info = result.unwrap();
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");
}

#[test]
fn default_builder_success() {
    let result = AdditionalInformationBuilder::default()
        .append("This is a test")
        .try_build();

    is_ok!(&result);

    let additional_info = result.unwrap();
    assert_eq!(additional_info.count(), 1);
}

#[test]
fn add_duplicate_success() {
    let result = AdditionalInformation::builder()
        .append("This is a test")
        .append("This is a test")
        .try_build();

    is_ok!(&result);

    let additional_info = result.unwrap();
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");
}

#[test]
fn add_error() {
    let result = AdditionalInformation::builder().append(" ").try_build();

    kernel_error_contains!(
        &result,
        Kind::InvalidInput,
        Audience::User,
        "We could not add the additional information ' '. "
    );
}
