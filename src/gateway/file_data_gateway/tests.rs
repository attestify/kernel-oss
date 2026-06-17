//! Verifies the legacy file-data compatibility alias.
//!
//! Bounded unit under test:
//! - `FileDataGateway`
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

use crate::error::{Error, Kind};
use crate::gateway::file_data_gateway::FileDataGateway;
use test_framework_oss::is_ok;

fn file_data_gateway_fixture(file_path: &str) -> Result<Vec<u8>, Error> {
    if file_path == "/tmp/evidence.txt" {
        return Ok(b"evidence".to_vec());
    }

    Err(Error::for_system(
        Kind::NotFound,
        "The requested file data was not found.",
    ))
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the legacy alias can be used as a callable function pointer.
#[test]
fn file_data_gateway_alias_success() {
    let retrieve: FileDataGateway = file_data_gateway_fixture;

    let actual = is_ok!(retrieve("/tmp/evidence.txt"));

    assert_eq!(actual, b"evidence".to_vec());
}
