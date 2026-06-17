//! Verifies the bounded kind value object.
//!
//! Bounded unit under test:
//! - `Kind`
//!
//! Public interfaces verified:
//! - `Kind::new`
//! - `Kind::to_string`
//!
//! Logical paths covered:
//! - supported kinds parse successfully
//! - empty and unsupported kinds fail validation
//! - both supported kinds format correctly
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::Kind;
use crate::error;
use crate::error::Error;

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a supported kind parses successfully.
#[test]
fn new_kind_success() {
    let result = Kind::new("AssuranceReport");
    assert_eq!(result, Ok(Kind::AssuranceReport));
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty kind values are rejected.
#[test]
fn new_kind_empty_error() {
    let result = Kind::new("");
    assert_eq!(
        result,
        Err(Error::for_user(
            error::Kind::InvalidInput,
            "You have provided an empty Kind value. Please provide a Kind value.".to_string()
        ))
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that unsupported kinds are rejected.
#[test]
fn new_kind_invalid_input_error() {
    let kind = Kind::new("invalid");
    assert_eq!(
        kind,
        Err(Error::for_user(
            error::Kind::InvalidInput,
            "'invalid' is not a valid Kind. Must be one of: [AssuranceReport, AssuranceProcedure]."
                .to_string()
        ))
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that `AssuranceReport` formats correctly.
#[test]
fn assurance_report_to_string_success() {
    let kind = Kind::AssuranceReport;
    assert_eq!(kind.to_string(), "AssuranceReport");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that `AssuranceProcedure` formats correctly.
#[test]
fn assurance_procedure_to_string_success() {
    let kind = Kind::AssuranceProcedure;
    assert_eq!(kind.to_string(), "AssuranceProcedure");
}
