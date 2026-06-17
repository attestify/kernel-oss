//! Verifies the ULID base32 codec helpers.
//!
//! Bounded unit under test:
//! - `encode`
//! - `decode`
//! - `LOOKUP`
//! - `ALPHABET`
//! - `ULID_LEN`
//!
//! Public interfaces verified:
//! - codec lookup-table generation
//! - valid ULID byte encoding
//! - valid ULID string decoding
//! - invalid length and invalid character handling
//!
//! Logical paths covered:
//! - valid uppercase and lowercase encoding/decoding round-trips
//! - lookup table entries cover uppercase and lowercase alphabet values
//! - empty and non-exact-length inputs fail validation
//! - invalid characters fail validation
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::ulid::base32::ALPHABET;
use crate::ulid::base32::DecodeError;
use crate::ulid::base32::ULID_LEN;
use crate::ulid::base32::{decode, encode};
use test_framework_oss::is_ok;

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the lookup table maps the expected bytes for the supported
/// ULID alphabet.
#[test]
fn lookup_table_success() {
    let mut lookup = [crate::ulid::base32::NO_VALUE; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lookup[c as usize] = i as u8;
        if !(c as char).is_numeric() {
            //lowercase
            lookup[(c + 32) as usize] = i as u8;
        }
    }
    assert_eq!(crate::ulid::base32::LOOKUP, lookup);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that valid ULID byte inputs encode and decode successfully.
#[test]
fn valid_chars_success() {
    let val = 0x41414141414141414141414141414141;
    assert_eq!(is_ok!(decode("21850M2GA1850M2GA1850M2GA1")), val);
    assert_eq!(encode(val), "21850M2GA1850M2GA1850M2GA1");

    let val = 0x4d4e385051444a59454234335a413756;
    let enc = "2D9RW50MA499CMAGHM6DD42DTP";
    let lower = enc.to_lowercase();
    assert_eq!(encode(val), enc);
    assert_eq!(is_ok!(decode(enc)), val);
    assert_eq!(is_ok!(decode(&lower)), val);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that encoded ULIDs always retain the expected output length and
/// that invalid-length inputs are rejected.
#[test]
fn length_success() {
    assert_eq!(encode(0xffffffffffffffffffffffffffffffff).len(), ULID_LEN);
    assert_eq!(encode(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f).len(), ULID_LEN);
    assert_eq!(encode(0x00000000000000000000000000000000).len(), ULID_LEN);

    assert_eq!(decode(""), Err(DecodeError::InvalidLength));
    assert_eq!(
        decode("2D9RW50MA499CMAGHM6DD42DT"),
        Err(DecodeError::InvalidLength)
    );
    assert_eq!(
        decode("2D9RW50MA499CMAGHM6DD42DTPP"),
        Err(DecodeError::InvalidLength)
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that encoded ULID output uses only supported alphabet characters
/// and that invalid characters are rejected during decoding.
#[test]
fn chars_success() {
    for ref c in encode(0xffffffffffffffffffffffffffffffff).bytes() {
        assert!(ALPHABET.contains(c));
    }
    for ref c in encode(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f).bytes() {
        assert!(ALPHABET.contains(c));
    }
    for ref c in encode(0x00000000000000000000000000000000).bytes() {
        assert!(ALPHABET.contains(c));
    }

    assert_eq!(
        decode("2D9RW50[A499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
    assert_eq!(
        decode("2D9RW50LA499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
    assert_eq!(
        decode("2D9RW50IA499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
}
