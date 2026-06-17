//! Tests for `Outcome`, covering parsing, formatting, and default selection behavior.
//!
//! Bounded unit under test: `Outcome`.
//! Public interfaces verified: `try_from`, `to_string`, and `default`.
//! Logical paths covered: valid inputs, empty input, invalid input, uppercase normalization, and
//! the default fallback value.
//! Requirement validation points: standards-aligned outcome parsing and formatting behavior.

use super::Outcome;
use crate::error;
use crate::error::Error;
use test_framework_oss::is_ok;

#[test]
/// Requirement validation: verifies `Outcome::try_from` accepts a valid input string.
fn new_outcome_success() {
    let result = Outcome::try_from("fail");
    assert_eq!(result, Ok(Outcome::FAIL));
}
#[test]
/// Requirement validation: verifies `Outcome::try_from` rejects an empty input string.
fn new_outcome_empty_error() {
    let result = Outcome::try_from("");
    assert_eq!(
        result,
        Err(Error::for_user(
            error::Kind::InvalidInput,
            "You have provided an empty Outcome value. Please provide an Outcome value."
                .to_string()
        ))
    );
}

#[test]
/// Requirement validation: verifies `Outcome::try_from` rejects an unsupported input string.
fn new_outcome_invalid_input_error() {
    let outcome = Outcome::try_from("invalid");
    assert_eq!(
        outcome,
        Err(Error::for_user(
            error::Kind::InvalidInput,
            "'invalid' is not a valid Outcome. Must be one of: [fail, inconclusive, pass, error]."
                .to_string()
        ))
    );
}

#[test]
/// Requirement validation: verifies the fail outcome renders as `fail`.
fn fail_to_string_success() {
    let outcome = Outcome::FAIL;
    assert_eq!(outcome.to_string(), "fail");
}

#[test]
/// Requirement validation: verifies the inconclusive outcome renders as `inconclusive`.
fn inconclusive_to_string_success() {
    let outcome = Outcome::INCONCLUSIVE;
    assert_eq!(outcome.to_string(), "inconclusive");
}

#[test]
/// Requirement validation: verifies the pass outcome renders as `pass`.
fn pass_to_string_success() {
    let outcome = Outcome::PASS;
    assert_eq!(outcome.to_string(), "pass");
}

#[test]
/// Requirement validation: verifies the error outcome renders as `error`.
fn error_to_string_success() {
    let outcome = Outcome::ERROR;
    assert_eq!(outcome.to_string(), "error");
}

#[test]
/// Requirement validation: verifies the default outcome is inconclusive.
fn default_outcome_success() {
    let outcome = Outcome::default();
    assert_eq!(outcome, Outcome::INCONCLUSIVE);
}

#[test]
/// Requirement validation: verifies mixed-case inputs normalize to the expected outcomes.
fn uppercase_input_success() {
    let pass_outcome = is_ok!(Outcome::try_from("PASS"));
    let fail_outcome = is_ok!(Outcome::try_from("Fail"));
    let inconclusive_outcome = is_ok!(Outcome::try_from("InCoNcLuSiVe"));
    let error_outcome = is_ok!(Outcome::try_from("ERROR"));
    assert_eq!(pass_outcome, Outcome::PASS);
    assert_eq!(fail_outcome, Outcome::FAIL);
    assert_eq!(inconclusive_outcome, Outcome::INCONCLUSIVE);
    assert_eq!(error_outcome, Outcome::ERROR);
}
