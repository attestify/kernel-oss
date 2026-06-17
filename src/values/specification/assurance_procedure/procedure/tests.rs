//! Tests for assurance-procedure `Procedure`, covering construction and validation behavior.
//!
//! Bounded unit under test: `assurance_procedure::Procedure`.
//! Public interfaces verified: `new`.
//! Logical paths covered: valid construction and invalid NRN, short description, and long
//! description inputs.
//! Requirement validation points: standards-aligned procedure value-object behavior.

use super::Procedure;
use crate::error;
use crate::values::nrn::NRN;
use crate::values::specification::description::Description;
use crate::values::specification::short_description::ShortDescription;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies `Procedure::new` accepts valid inputs.
fn new_procedure_success() {
    let result = Procedure::new(
        "nrn:sourcecode:example",
        "Short Description",
        "Long Description",
    );

    let procedure = is_ok!(result);
    assert_eq!(procedure.nrn, is_ok!(NRN::new("nrn:sourcecode:example")));
    assert_eq!(
        procedure.short,
        is_ok!(ShortDescription::try_from("Short Description"))
    );
    assert_eq!(
        procedure.description,
        is_ok!(Description::try_from("Long Description"))
    );
}

#[test]
/// Requirement validation: verifies invalid NRNs are rejected.
fn new_procedure_invalid_nrn_error() {
    let result = Procedure::new("invalid-nrn", "Short Description", "Long Description");

    let result = is_error!(result);
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(
        result
            .message
            .starts_with("There is an issue with your procedure information: ")
    );
}

#[test]
/// Requirement validation: verifies invalid short descriptions are rejected.
fn new_procedure_invalid_short_description_error() {
    let result = Procedure::new("nrn:sourcecode:example", " ", "Long Description");

    let result = is_error!(result);
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(result.message.starts_with(
        "There is an issue with your procedure information: The short description has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies invalid long descriptions are rejected.
fn new_procedure_invalid_long_description_error() {
    let result = Procedure::new("nrn:sourcecode:example", "Short Description ", " ");

    let result = is_error!(result);
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(result.message.starts_with(
        "There is an issue with your procedure information: The description has an issue: "
    ));
}
