//! Verifies the legacy retrieve-directory-path compatibility alias.
//!
//! Bounded unit under test:
//! - `RetrieveDirectoryPath`
//!
//! Public interfaces verified:
//! - the alias resolves to a callable function type
//!
//! Logical paths covered:
//! - the alias can forward to a concrete implementation
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

#![allow(deprecated)]

use crate::gateway::directory_list::RetrieveDirectoryPath;
use test_framework_oss::is_ok;

fn retrieve_directory_path_fixture(directory_key: &str) -> Result<String, crate::error::Error> {
    if directory_key == "evidence" {
        return Ok("/tmp/evidence".to_string());
    }

    Err(crate::error::Error::for_system(
        crate::error::Kind::NotFound,
        "The requested directory path was not found.",
    ))
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the legacy alias can be used as a callable function pointer.
#[test]
fn retrieve_directory_path_alias_success() {
    let retrieve: RetrieveDirectoryPath = retrieve_directory_path_fixture;

    let actual = is_ok!(retrieve("evidence"));

    assert_eq!(actual, "/tmp/evidence");
}
