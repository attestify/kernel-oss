//! Tests for NRN parsing and NID/NSS helpers.
//!
//! Bounded unit under test: the NRN, NID, NSS, and NapeNID value objects.
//! Public interfaces verified: `NRN::new`, `NID::new`, `NSS::new`, `NapeNID::new`, and display
//! formatting for NIDs.
//! Logical paths covered: valid NRNs, scheme/NID/NSS validation failures, display conversion,
//! and NID registry checks.
//! Requirement validation points: standards-aligned parsing and validation behavior for the NRN
//! domain helpers.

use crate::error::{Audience, Kind};
use crate::values::nrn::{NID, NRN, NSS, NapeNID};
use test_framework_oss::{is_error, is_ok};

/*** NRN Tests ***/

#[test]
/// Requirement validation: verifies `NRN::new` parses a valid NRN value.
fn from_str_success() {
    let result = NRN::new("nrn:sourcecode:nape:project/nape-cli");

    let nrn = is_ok!(result);
    assert_eq!(nrn.scheme, "nrn");
    assert_eq!(nrn.nid, NapeNID::SourceCode);
    assert_eq!(nrn.nss[0], is_ok!(NSS::new("nape")));
    assert_eq!(nrn.nss[1], is_ok!(NSS::new("project/nape-cli")));
}
#[test]
/// Requirement validation: verifies `NRN::new` rejects an empty NRN value.
fn from_str_empty_error() {
    let result = NRN::new("");

    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "You provided an empty NRN value. An NRN must be in the format 'nrn:<nid>:<nss>'"
    );
}
#[test]
/// Requirement validation: verifies `NRN::new` rejects an invalid scheme.
fn from_str_wrong_scheme_error() {
    let result = NRN::new("urn:sourecode:nape::project/nape-cli");

    let error = is_error!(result);
    assert_eq!(
        error.message,
        "You provided 'urn:sourecode:nape::project/nape-cli' as an NRN and the scheme 'urn'  is not valid. Must be 'nrn'"
    );
}
#[test]
/// Requirement validation: verifies `NRN::new` rejects an unknown NID.
fn from_str_wrong_nid_error() {
    let result = NRN::new("nrn:somewrongnid:nape::project/nape-cli");

    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "'somewrongnid' is not a valid NID. Must be one of: [procedure, sourcecode]."
    );
}
#[test]
/// Requirement validation: verifies `NRN::new` rejects NRNs missing NSS segments.
fn from_str_missing_nss_error() {
    let result = NRN::new("nrn:sourcecode");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The supplied NRN 'nrn:sourcecode' must have at least one NSS after the NID."
    );
}
#[test]
/// Requirement validation: verifies `NRN::new` rejects invalid NSS content.
fn from_str_invalid_nss_error() {
    let result = NRN::new("nrn:sourcecode:nape:project/nape-cli:invalid nss");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "Input 'invalid nss' contains whitespace character ' ' at position 7"
    );
}

/** NAPE NID Tests ***/

#[test]
/// Requirement validation: verifies NID display formatting for the `SourceCode` variant.
fn sourcecode_display_success() {
    let result = format!("{}", NapeNID::SourceCode);
    assert_eq!(result, "sourcecode");
    assert_eq!(result.to_string(), "sourcecode");
}
#[test]
/// Requirement validation: verifies NID display formatting for the `Procedure` variant.
fn procedure_display_success() {
    let result = format!("{}", NapeNID::Procedure);
    assert_eq!(result, "procedure");
    assert_eq!(result.to_string(), "procedure");
}

#[test]
/// Requirement validation: verifies `NapeNID::new` rejects an unknown NID.
fn missing_nid_error() {
    let nid = is_ok!(NID::new("does-not-exist"));
    let does_not_exist = NapeNID::new(nid);

    let dne_error = is_error!(does_not_exist);
    assert_eq!(dne_error.kind, Kind::InvalidInput);
    assert_eq!(dne_error.audience, Audience::User);
    assert_eq!(
        dne_error.message,
        "'does-not-exist' is not a valid NID. Must be one of: [procedure, sourcecode]."
    );
}
#[test]
/// Requirement validation: verifies `NapeNID::new` accepts supported NIDs.
fn supported_nid_success() {
    let sourcecode_nid = NapeNID::new(is_ok!(NID::new("sourcecode")));
    let procedure_nid = NapeNID::new(is_ok!(NID::new("procedure")));

    assert_eq!(is_ok!(sourcecode_nid), NapeNID::SourceCode);
    assert_eq!(is_ok!(procedure_nid), NapeNID::Procedure);
}

/*** NID Tests ***/

#[test]
/// Requirement validation: verifies `NID::new` accepts a valid NID.
fn new_nid_success() {
    let result = NID::new("sourcecode");
    let nid = is_ok!(result);
    assert_eq!(nid.value, "sourcecode");
}
#[test]
/// Requirement validation: verifies `NID::new` rejects an empty string.
fn new_nid_empty_error() {
    let result = NID::new("");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The NID provided is blank.");
}
#[test]
/// Requirement validation: verifies `NID::new` rejects a leading numeric character.
fn new_nid_first_char_not_alphabetical_error() {
    let result = NID::new("1sourcecode");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The NID must start with an alphabetic character. You NID '1sourcecode' starts with '1'."
    );
}
#[test]
/// Requirement validation: verifies `NID::new` rejects invalid characters.
fn new_nid_invalid_character_error() {
    let result = NID::new("source code");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "NID 'source code' contains invalid character ' ' at position 6.  An NID can only contains alphanumeric characters or a dash '-'."
    )
}
#[test]
/// Requirement validation: verifies `NID::new` enforces the maximum length.
fn new_nid_too_long_error() {
    let result = NID::new("sourcecode1234567890123456789012");
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(
        error.message,
        "The NID 'sourcecode1234567890123456789012' is too long. An NID must be 31 characters or less."
    )
}

/*** NSS Tests ***/

#[test]
/// Requirement validation: verifies `NSS::new` accepts a valid NSS value.
fn new_nss_success() {
    let result = NSS::new("test");
    is_ok!(result);
}
#[test]
/// Requirement validation: verifies `NSS::new` rejects whitespace.
fn new_nss_whitespace_error() {
    let result = NSS::new("test test");
    let error = is_error!(result);
    assert_eq!(
        error.message,
        "Input 'test test' contains whitespace character ' ' at position 4"
    );
}
#[test]
/// Requirement validation: verifies `NSS::new` rejects non-ASCII input.
fn new_nss_non_ascii_error() {
    let result = NSS::new("test😀");
    let error = is_error!(result);
    assert_eq!(
        error.message,
        "Input 'test😀' contains non-ASCII character '😀' at position 4"
    );
}
