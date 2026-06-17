//! Verifies the bounded signature-algorithm value object.
//!
//! Bounded unit under test:
//! - `SignatureType`
//! - `Signature`
//!
//! Public interfaces verified:
//! - `SignatureType::from`
//! - `SignatureType::to_string`
//! - `Signature::try_new`
//! - `Signature::try_from`
//! - `Signature::to_string`
//!
//! Logical paths covered:
//! - supported algorithm parsing succeeds
//! - unsupported algorithm parsing fails
//! - display formatting preserves the canonical algorithm label
//! - empty signature input fails validation
//! - valid structured signature parsing succeeds
//! - malformed structured signature input fails in each bracket/error case
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::algorithms::signature_algorithm::{Signature, SignatureType};
use crate::error::{Audience, Kind};
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_eq;

mod signature_type {
    use super::*;

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that unsupported signature algorithms are rejected.
    #[test]
    fn new_signature_type_error_from_str() {
        let sig_type = SignatureType::from("some-invalid-type");

        kernel_error_eq!(
            sig_type,
            Kind::InvalidInput,
            Audience::System,
            "The signature algorithm 'some-invalid-type' is not supported. The supported algorithms are: [SHA256]"
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that the supported algorithm string parses to `SHA256`.
    #[test]
    fn new_signature_type_sha256_from_str() {
        let sig_type = is_ok!(SignatureType::from("SHA256"));
        assert_eq!(sig_type, SignatureType::SHA256);
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that `SignatureType::SHA256` formats as `SHA256`.
    #[test]
    fn sha256_display_success() {
        let sha256 = SignatureType::SHA256;
        assert_eq!(sha256.to_string(), "SHA256");
    }
}

mod signature {
    use super::*;

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that constructing a signature from a type and payload preserves
    /// both the canonical structure and the raw payload.
    #[test]
    fn try_new_success() {
        let sig = Signature::try_new(SignatureType::SHA256, "signature");
        let sig = is_ok!(sig);

        assert_eq!(sig.structure_signature(), "SHA256[signature]");
        assert_eq!(sig.signature_type(), &SignatureType::SHA256);
        assert_eq!(sig.to_string(), "signature");
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that empty signature payloads are rejected.
    #[test]
    fn try_new_empty_string_error() {
        let result = Signature::try_new(SignatureType::SHA256, "");
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature you provided is empty. Please provide a non-empty signature value."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that a structured signature string parses to the expected
    /// bounded signature.
    #[test]
    fn try_from_success() {
        let signature_str = "SHA256[234928039042340892]";
        let result = Signature::try_from(signature_str);
        let signature = is_ok!(result);
        assert_eq!(signature.signature_type(), &SignatureType::SHA256);
        assert_eq!(signature.to_string(), "234928039042340892");
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that unsupported structured signature algorithms are rejected.
    #[test]
    fn try_from_unsupported_algorithm_error() {
        let signature_str = "BILL123[234928039042340892]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature algorithm 'BILL123' is not supported. The supported algorithms are: [SHA256]"
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that signatures missing the algorithm segment are rejected.
    #[test]
    fn try_from_no_algorithm_error() {
        let signature_str = "[234928039042340892]";
        let result = Signature::try_from(signature_str);

        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature string provided does not have a signature algorithm. The signature algorithm are the characters before the first bracket: ALGO[the-signature-data-here]."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that signatures missing payload data are rejected.
    #[test]
    fn try_from_empty_signature_error() {
        let signature_str = "SHA256[]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature string provided does not have any signature data. Signature data are the characters between the two brackets: ALGO[the-signature-data-here]."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that signatures missing the opening bracket are rejected.
    #[test]
    fn try_from_no_first_bracket_error() {
        let signature_str = "SHA256234928039042340892]";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256234928039042340892]' is not in the correct format, the first bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data]."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that signatures missing the closing bracket are rejected.
    #[test]
    fn try_from_no_last_bracket_error() {
        let signature_str = "SHA256[234928039042340892";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256[234928039042340892' is not in the correct format, the last bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data]."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that signatures with reversed brackets are rejected.
    #[test]
    fn try_from_backwards_brackets() {
        let signature_str = "SHA256]234928039042340892[";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature you provided has the closing bracket before the opening bracket. Check the signature you provided to ensure it in the format of ALGO[the-signature-data]."
        );
    }

    /// Requirement validation: No requirement validation point is currently supplied.
    ///
    /// Verifies that structured signatures without any brackets are rejected.
    #[test]
    fn try_from_no_brackets_error() {
        let signature_str = "SHA256234928039042340892";
        let result = Signature::try_from(signature_str);
        kernel_error_eq!(
            result,
            Kind::InvalidInput,
            Audience::System,
            "The signature 'SHA256234928039042340892' is not in the correct format, the first bracket is missing. Check the signature you provided to ensure it in the format of ALGO[the-signature-data]."
        );
    }
}
