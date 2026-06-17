//! Tests for assurance-report `SignedFile`, covering construction and accessors.
//!
//! Bounded unit under test: `assurance_report::SignedFile`.
//! Public interfaces verified: `new`, `file`, `signature`, and `signature_type`.
//! Logical paths covered: valid signed-file construction and invalid file-path rejection.
//! Requirement validation points: standards-aligned signed-file behavior for assurance reports.

use crate::algorithms::signature_algorithm::{Signature, SignatureType::SHA256};
use crate::error::{Audience, Kind};
use crate::values::specification::assurance_report::signed_file::SignedFile;
use test_framework_oss::{is_error, is_ok};

fn signature() -> Signature {
    is_ok!(Signature::try_new(SHA256, "the-signature"))
}

#[test]
/// Requirement validation: verifies `SignedFile::new` accepts a valid file path and signature.
fn new_success() {
    let signature = signature();
    let signed_file = is_ok!(SignedFile::new("./some-location/file.txt", &signature));

    assert_eq!(signed_file.file().as_str(), "./some-location/file.txt");
    assert_eq!(signed_file.signature(), &signature);
    assert_eq!(signed_file.signature_type(), &SHA256);
}

#[test]
/// Requirement validation: verifies `SignedFile::new` rejects an empty file path.
fn new_error_empty_file_path() {
    let signature = signature();
    let result = SignedFile::new("", &signature);

    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(
        error
            .message
            .starts_with("The file path is required, although a value was not provided.")
    );
}
