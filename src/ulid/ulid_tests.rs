//! Verifies the bounded ULID value object.
//!
//! Bounded unit under test:
//! - `ULID`
//!
//! Public interfaces verified:
//! - `ULID::from_string`
//! - `ULID::from_str`
//! - `ULID::increment`
//! - `ULID::default`
//! - `Display`
//! - conversion traits into string, integer, tuple, and bytes
//!
//! Logical paths covered:
//! - string parsing succeeds for valid ULID values
//! - string parsing supports canonical and alternate forms used by the module
//! - incrementing succeeds until the bounded maximum is reached
//! - increment overflow returns no next value
//! - display and conversion traits preserve the same ULID
//! - default returns the nil ULID
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::ulid::ULID;
use crate::ulid::base32::DecodeError;
use crate::ulid::base32::EncodeError;
use std::str::FromStr;
use test_framework_oss::is_ok;

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a ULID round-trips through the internal string form.
#[test]
fn static_success() {
    let s = ULID(0x41414141414141414141414141414141).to_string();
    let u = is_ok!(ULID::from_string(&s));
    assert_eq!(&s, "21850M2GA1850M2GA1850M2GA1");
    assert_eq!(u.0, 0x41414141414141414141414141414141);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that incrementing a ULID advances to the next bounded value until
/// the maximum is reached.
#[test]
fn increment_success() {
    let ulid = is_ok!(ULID::from_string("01BX5ZZKBKAZZZZZZZZZZZZZZZ"));
    let ulid = match ulid.increment() {
        Some(value) => value,
        None => panic!("Expected ULID increment to return a next value."),
    };
    assert_eq!("01BX5ZZKBKB000000000000000", ulid.to_string());

    let ulid = is_ok!(ULID::from_string("01BX5ZZKBKZZZZZZZZZZZZZZZX"));
    let ulid = match ulid.increment() {
        Some(value) => value,
        None => panic!("Expected ULID increment to return a next value."),
    };
    assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZY", ulid.to_string());
    let ulid = match ulid.increment() {
        Some(value) => value,
        None => panic!("Expected ULID increment to return a next value."),
    };
    assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZZ", ulid.to_string());
    assert!(ulid.increment().is_none());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that incrementing the maximum ULID produces no next value.
#[test]
fn increment_overflow_error() {
    let ulid = ULID(u128::MAX);
    assert!(ulid.increment().is_none());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the conversion traits all preserve the same ULID value.
#[test]
fn into_string_success() {
    let ulid = is_ok!(ULID::from_str("01FKMG6GAG0PJANMWFN84TNXCD"));
    let s: String = ulid.into();
    let u: u128 = ulid.into();
    let uu: (u64, u64) = ulid.into();
    let bytes: [u8; 16] = ulid.into();

    assert_eq!(is_ok!(ULID::from_str(&s)), ulid);
    assert_eq!(ULID::from(u), ulid);
    assert_eq!(ULID::from(uu), ulid);
    assert_eq!(ULID::from(bytes), ulid);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the default ULID is the nil ULID.
#[test]
fn default_is_nil_success() {
    assert_eq!(ULID::default(), ULID::nil());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the ULID and its helper error types remain displayable.
#[test]
fn display_success() {
    println!("{}", ULID::nil());
    println!("{}", EncodeError::BufferTooSmall);
    println!("{}", DecodeError::InvalidLength);
    println!("{}", DecodeError::InvalidChar);
}
