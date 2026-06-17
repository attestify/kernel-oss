//! Verifies the bounded subject value object.
//!
//! Bounded unit under test:
//! - `Subject`
//!
//! Public interfaces verified:
//! - `Subject::try_new`
//! - `Subject::nrn`
//! - `Subject::id`
//!
//! Logical paths covered:
//! - valid NRN and subject ID pairs succeed
//! - invalid NRN input fails
//! - invalid subject ID input fails
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::Subject;
use crate::error::{Audience, Kind};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a valid subject parses successfully.
#[test]
fn new_subject_success() {
    let nrn = "nrn:procedure:example";
    let id = "example";
    let result = Subject::try_new(nrn, id);

    let subject = is_ok!(result);
    assert_eq!(subject.nrn().value(), nrn);
    assert_eq!(subject.id().value(), id);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that an invalid NRN is rejected.
#[test]
fn new_subject_error_handle_nrn_error() {
    let nrn = "bad-nrn";
    let id = "example";
    let result = Subject::try_new(nrn, id);

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .contains("We were unable to create the Subject: ")
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that an invalid subject ID is rejected.
#[test]
fn new_subject_error_handle_subject_id_error() {
    let nrn = "nrn:procedure:example";
    let id = "a-bad-id!";
    let result = Subject::try_new(nrn, id);

    let error = is_error!(result);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(
        error
            .message
            .contains("We were unable to create the Subject: ")
    );
}
