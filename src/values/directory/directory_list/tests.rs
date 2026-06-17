//! Tests for `DirectoryList`, covering construction, insertion, lookup, and merge behavior.
//!
//! Bounded unit under test: `DirectoryList`.
//! Public interfaces verified: `default`, `try_from_vec`, `try_add`, `try_get`, and `try_merge`.
//! Logical paths covered: empty inputs, successful insertions, duplicate key handling, missing
//! lookups, and list merge behavior.
//! Requirement validation points: standards-aligned value-object coverage for directory path
//! collection behavior.

use super::DirectoryList;
use crate::error::{Audience, Kind};
use std::collections::HashMap;
use test_framework_oss::{is_error, is_ok};

/*** Happy Path Tests ***/
#[test]
/// Requirement validation: verifies the default `DirectoryList` starts empty.
fn directory_list_default_success() {
    let directory_list = DirectoryList::default();
    assert_eq!(directory_list.paths.len(), 0);
}
#[test]
/// Requirement validation: verifies `try_from_vec` accepts a populated path list.
fn directory_list_from_vec_success() {
    let paths = vec![("path-key".to_string(), "/path/to/file".to_string())];
    let result = DirectoryList::try_from_vec(paths);

    let list = is_ok!(result);
    assert_eq!(list.paths.len(), 1);
}

#[test]
/// Requirement validation: verifies `try_from_hashmap` accepts a populated path map.
fn directory_list_from_hashmap_success() {
    let mut paths = HashMap::new();
    paths.insert("path-key".to_string(), "/path/to/file".to_string());

    let result = DirectoryList::try_from_hashmap(paths);

    let list = is_ok!(result);
    assert_eq!(list.paths.len(), 1);
    assert_eq!(list.try_get("path-key"), Some("/path/to/file".to_string()));
}

#[test]
/// Requirement validation: verifies `try_from_hashmap` preserves an empty input as an empty list.
fn directory_list_from_empty_hashmap_success() {
    let paths = HashMap::new();

    let result = DirectoryList::try_from_hashmap(paths);

    let list = is_ok!(result);
    assert_eq!(list.paths.len(), 0);
}
#[test]
/// Requirement validation: verifies `try_add` inserts a new path entry.
fn directory_list_add_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);
    assert_eq!(directory_list.paths.len(), 1);
}
#[test]
/// Requirement validation: verifies `try_add` supports repeated successful insertions.
fn directory_list_add_multiple_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);
    assert_eq!(directory_list.paths.len(), 1);

    let result = directory_list.try_add("path-key-2", "/path/to/directory-2");

    let directory_list = is_ok!(result);
    assert_eq!(directory_list.paths.len(), 2);
}
#[test]
/// Requirement validation: verifies `try_get` returns a stored path when the key exists.
fn directory_list_get_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);

    let result = directory_list.try_get("path-key");
    assert_eq!(result, Some("/path/to/directory".to_string()));
}

#[test]
/// Requirement validation: verifies `try_merge` combines distinct directory lists.
fn try_merge_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);

    let other_directory_list = DirectoryList::default();
    let result = other_directory_list.try_add("path-key-2", "/path/to/directory-2");

    let other_directory_list = is_ok!(result);

    let result = directory_list.try_merge(&other_directory_list);

    let merged_directory_list = is_ok!(result);
    assert_eq!(merged_directory_list.paths.len(), 2);
}

#[test]
/// Requirement validation: verifies `try_merge` rejects duplicate path names across lists.
fn try_merge_duplicate_path_name_error() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);

    let other_directory_list = DirectoryList::default();
    let result = other_directory_list.try_add("path-key", "/path/to/other-directory");

    let other_directory_list = is_ok!(result);

    let result = directory_list.try_merge(&other_directory_list);

    let error = is_error!(result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(
        error.message,
        "The path name 'path-key' already exists in the directory List.  Please provide a unique path name."
    );
}

/*** Sad Path Tests ***/
#[test]
/// Requirement validation: verifies `try_from_vec` rejects an empty path list.
fn directory_list_from_empty_vec_error() {
    let paths = vec![];
    let result = DirectoryList::try_from_vec(paths);

    let error = is_error!(result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(
        error.message,
        "No directory or file paths have been provided."
    );
}
#[test]
/// Requirement validation: verifies `try_add` rejects an empty path name.
fn directory_list_add_empty_path_name_error() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("", "/path/to/directory");

    let error = is_error!(result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(
        error.message,
        "You provided an empty path name to add to the directory List.  Please provide a non-empty path name."
    );
}
#[test]
/// Requirement validation: verifies `try_add` rejects duplicate path names.
fn directory_list_add_duplicate_path_name_error() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);

    let result = directory_list.try_add("path-key", "/path/to/directory-2");

    let error = is_error!(result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(
        error.message,
        "The path name 'path-key' already exists in the directory List.  Please provide a unique path name."
    );
}
#[test]
/// Requirement validation: verifies `try_add` rejects an empty path value.
fn directory_list_add_empty_path_error() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "");

    let error = is_error!(result);

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(
        error.message,
        "You provided an empty path to add to the directory List.  Please provide a non-empty path."
    );
}
#[test]
/// Requirement validation: verifies `try_get` returns `None` when the key is missing.
fn directory_list_get_not_found_error() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    let directory_list = is_ok!(result);

    let result = directory_list.try_get("path-key-2");
    assert_eq!(result, None);
}
