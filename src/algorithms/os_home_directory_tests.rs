//! Verifies the operating-system home-directory helper.
//!
//! Bounded unit under test:
//! - `retrieve_from_env`
//!
//! Public interfaces verified:
//! - `retrieve_from_env`
//! - `retrieve`
//!
//! Logical paths covered:
//! - `HOME` wins when present
//! - `USERPROFILE` is used when `HOME` is missing
//! - missing environment values return a bounded error
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::algorithms::os_home_directory::retrieve_from_env;
use crate::error::{Audience, Kind};
use std::path::PathBuf;
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the home directory value wins when both environment inputs are present.
#[test]
fn retrieve_from_env_home_success() {
    let result = retrieve_from_env(
        Some("/home/tester".to_string()),
        Some("C:\\Users\\tester".to_string()),
    );

    assert_eq!(is_ok!(result), PathBuf::from("/home/tester"));
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that `USERPROFILE` is used when `HOME` is unavailable.
#[test]
fn retrieve_from_env_userprofile_success() {
    let result = retrieve_from_env(None, Some("C:\\Users\\tester".to_string()));

    assert_eq!(is_ok!(result), PathBuf::from("C:\\Users\\tester"));
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that missing environment inputs produce a bounded not-found error.
#[test]
fn retrieve_from_env_missing_error() {
    let error = is_error!(retrieve_from_env(None, None));

    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.kind, Kind::NotFound);
    assert_eq!(error.message, "Home directory not found");
}
