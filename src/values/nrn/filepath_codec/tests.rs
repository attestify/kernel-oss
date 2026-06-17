//! Tests for NRN directory-name encoding and decoding helpers.
//!
//! Bounded unit under test: the NRN filepath codec helpers.
//! Public interfaces verified: `encode_as_directory_name` and `decode_from_directory_name`.
//! Logical paths covered: valid encode/decode round-trips and invalid directory-name decoding.
//! Requirement validation points: standards-aligned translation between NRN values and directory
//! names for filesystem use.

use super::{decode_from_directory_name, encode_as_directory_name};
use crate::values::nrn::NRN;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies an NRN encodes into a directory-safe name.
fn encode_success() {
    let nrn = is_ok!(NRN::new(
        "nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration"
    ));
    let result = encode_as_directory_name(&nrn);
    assert_eq!(
        result,
        "nrn_procedure_nape_-_software-procedures_rust-ci_-_sourcecode-integration"
    );
}

#[test]
/// Requirement validation: verifies a directory-safe name decodes back into the original NRN.
fn decode_success() {
    let result = is_ok!(decode_from_directory_name(
        "nrn_procedure_nape_-_software-procedures_rust-ci_-_sourcecode-integration",
    ));
    assert_eq!(
        result.value,
        "nrn:procedure:nape/software-procedures:rust-ci/sourcecode-integration"
    );
}

#[test]
/// Requirement validation: verifies invalid directory names are rejected during decoding.
fn decode_error_bad_nrn() {
    let result = decode_from_directory_name("sourcecode_example");
    let error = is_error!(result);

    assert_eq!(error.kind, crate::error::Kind::InvalidInput);
    assert_eq!(error.audience, crate::error::Audience::User);
    assert!(
        error.message.starts_with(
            "Could not decode the NRN from the directory name: 'sourcecode_example': "
        )
    );
}
