//! Tests for `MetaData`, covering insert, update, removal, and validation behavior.
//!
//! Bounded unit under test: `MetaData`.
//! Public interfaces verified: `default`, `add`, `get`, `remove`, and `upsert`.
//! Public interfaces verified: `data` accessor.
//! Logical paths covered: empty state, add/update/remove success, duplicate-key handling, missing
//! key lookup, oversized inputs, and invalid key/value inputs.
//! Requirement validation points: standards-aligned metadata collection behavior.

use super::{KEY_MAX, MetaData, VALUE_MAX};
use crate::error::{Audience, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::name::Name;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies the default metadata collection starts empty.
fn new_metadata_success() {
    let meta = MetaData::default();
    assert_eq!(meta.data.len(), 0);
}

#[test]
/// Requirement validation: verifies the metadata slice accessor returns the stored entries.
fn data_accessor_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");

    let data = meta.data();

    assert_eq!(data.len(), 1);
    assert_eq!(data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(data[0].1, is_ok!(Description::try_from("value")));
}

#[test]
/// Requirement validation: verifies adding a metadata entry stores the expected key/value pair.
fn add_to_metadata_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");
    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));
}

#[test]
/// Requirement validation: verifies metadata values can be retrieved after insertion.
fn get_metadata_value_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");
    assert_eq!(meta.get("key"), Some("value".to_string()));
}

#[test]
/// Requirement validation: verifies removing an existing metadata entry clears it from the list.
fn remove_from_metadata_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));

    meta.remove("key");

    assert_eq!(meta.data.len(), 0);
}

#[test]
/// Requirement validation: verifies removing a missing metadata entry leaves the list unchanged.
fn remove_a_key_that_does_not_exist_success() {
    let mut meta = MetaData::default();
    let _ = meta.add("key", "value");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));

    meta.remove("key-dne");

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));
}

#[test]
/// Requirement validation: verifies upsert replaces an existing metadata value.
fn upsert_metadata_success() {
    let mut meta = MetaData::default();
    let first_result = meta.add("key", "value");

    is_ok!(first_result);

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));

    let second_result = meta.upsert("key", "new-value");

    is_ok!(second_result);

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("new-value")));
}

#[test]
/// Requirement validation: verifies upsert adds a new metadata entry when the key is missing.
fn upsert_new_metadata_key_success() {
    let mut meta = MetaData::default();

    let result = meta.upsert("key", "value");

    is_ok!(result);

    assert_eq!(meta.data.len(), 1);
    assert_eq!(meta.data[0].0, is_ok!(Name::try_from("key")));
    assert_eq!(meta.data[0].1, is_ok!(Description::try_from("value")));
}

/** Key Error Tests */

#[test]
/// Requirement validation: verifies oversized metadata keys are rejected.
fn add_key_too_long_error() {
    let mut meta = MetaData::default();
    let key = "a".repeat(KEY_MAX + 1);
    let value = "value";
    let result = meta.add(key.as_str(), value);
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        format!(
            "The metadata key '{}' exceeds the maximum allowed length of '64' characters.  Please provide a metadata key that is less than '64' characters.",
            key
        )
    );
}

#[test]
/// Requirement validation: verifies invalid metadata keys are rejected.
fn add_invalid_key_error() {
    let mut meta = MetaData::default();
    let key = "a b";
    let value = "value";
    let result = meta.add(key, value);
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The metadata key 'a b' is invalid: ")
    );
}

/** Value Error Tests */

#[test]
/// Requirement validation: verifies oversized metadata values are rejected.
fn add_value_too_long_error() {
    let mut meta = MetaData::default();
    let key = "key";
    let value = "a".repeat(VALUE_MAX + 1);
    let result = meta.add(key, value.as_str());
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        format!(
            "The metadata value '{}' exceeds the maximum allowed length of '256' characters.  Please provide a metadata value that is less than '256' characters.",
            value
        )
    );
}

#[test]
/// Requirement validation: verifies invalid metadata values are rejected.
fn add_invalid_value_error() {
    let mut meta = MetaData::default();
    let key = "key";
    let value = "  ";
    let result = meta.add(key, value);
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The metadata value '  ' is invalid: ")
    );
}

/** Get Error Tests */

#[test]
/// Requirement validation: verifies lookup of a missing metadata key returns `None`.
fn get_missing_metadata_value_success() {
    let meta = MetaData::default();
    assert_eq!(meta.get("key-dne"), None);
}

/** Upsert Error Tests */

#[test]
/// Requirement validation: verifies invalid keys are rejected during upsert.
fn upsert_invalid_key_error() {
    let mut meta = MetaData::default();
    let result = meta.upsert("some bad key", "value");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The metadata key 'some bad key' is invalid: ")
    );
}

#[test]
/// Requirement validation: verifies invalid values are rejected during upsert.
fn upsert_invalid_value_error() {
    let mut meta = MetaData::default();
    let result = meta.upsert("good-key", "");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The metadata value '' is invalid: ")
    );
}
