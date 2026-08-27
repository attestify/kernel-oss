//! Tests for the checked private RFC 3492 Punycode primitive.

use super::{decode, encode};

#[test]
/// Verifies RFC 3492's canonical German-label example.
fn rfc_example_success() {
    assert_eq!(encode("bücher"), Ok("bcher-kva".to_owned()));
    assert_eq!(decode("bcher-kva"), Ok("bücher".to_owned()));
}

#[test]
/// Verifies basic-only input retains RFC 3492's trailing delimiter.
fn basic_only_delimiter_success() {
    assert_eq!(encode("example"), Ok("example-".to_owned()));
    assert_eq!(decode("example-"), Ok("example".to_owned()));
}

#[test]
/// Verifies mixed basic and non-basic input uses the canonical delimiter form.
fn mixed_basic_non_basic_delimiter_success() {
    assert_eq!(encode("mañana"), Ok("maana-pta".to_owned()));
}

#[test]
/// Verifies representative labels round trip through the independent primitive.
fn round_trip_success() {
    for label in ["bücher", "例え", "mañana", "a😀"] {
        let encoded = encode(label);
        assert!(encoded.is_ok(), "{label:?}");
        if let Ok(encoded) = encoded {
            assert_eq!(decode(&encoded), Ok(label.to_owned()), "{label:?}");
        }
    }
}

#[test]
/// Verifies invalid Punycode digits are rejected.
fn invalid_digit_error() {
    assert!(decode("bcher_").is_err());
}

#[test]
/// Verifies arithmetic-heavy malformed input returns an error without panicking.
fn overflow_error() {
    assert!(decode(&"z".repeat(100)).is_err());
}
