use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use super::{Error, Audience, Kind};

/// Test: new constructor, accessors and Display
/// What: Construct an error with `Error::new` and verify fields, `is_user`/`is_system`, and `Display`.
/// Why: Ensures basic construction and string display of the message works.
#[test]
fn new_error_success() {
    let e = Error::new(Audience::User, Kind::InvalidInput, "bad input");
    assert_eq!(e.audience, Audience::User);
    assert_eq!(e.kind, Kind::InvalidInput);
    assert_eq!(e.message, "bad input");
    assert!(e.is_user());
    assert!(!e.is_system());
    assert_eq!(format!("{}", e), "bad input");
}

/// Test: for_user and for_system constructors
/// What: Use the convenience constructors and validate audiences and booleans.
/// Why: Verifies the convenience constructors set the correct audience.
#[test]
fn convenience_constructors_set_audience_success() {
    let u = Error::for_user(Kind::NotFound, "not found");
    let s = Error::for_system(Kind::Unexpected, "oh no");
    assert!(u.is_user() && !u.is_system());
    assert!(s.is_system() && !s.is_user());
    assert_eq!(u.kind, Kind::NotFound);
    assert_eq!(s.kind, Kind::Unexpected);
}

/// Test: Clone, Eq and Hash produce consistent results
/// What: Clone an Error and compare equality, then hash both and compare hashes.
/// Why: Ensures derived `Clone`, `PartialEq`/`Eq`, and `Hash` are consistent.
#[test]
fn clone_eq_and_hash_consistency_success() {
    let a = Error::new(Audience::System, Kind::GatewayError, "gw fail");
    let b = a.clone();
    assert_eq!(a, b);

    let mut ha = DefaultHasher::new();
    a.hash(&mut ha);
    let ha = ha.finish();

    let mut hb = DefaultHasher::new();
    b.hash(&mut hb);
    let hb = hb.finish();

    assert_eq!(ha, hb);
}

/// Test: HashSet requires Hash and Eq
/// What: Insert an Error into a HashSet and assert an equal Error is contained.
/// Why: Enforces presence and correctness of `Hash` and `Eq`.
#[test]
fn hashset_contains_equal_success() {
    let a = Error::for_user(Kind::PermissionDenied, "denied");
    let b = Error::for_user(Kind::PermissionDenied, "denied");
    let mut set: HashSet<Error> = HashSet::new();
    set.insert(a.clone());
    assert!(set.contains(&b));
}

/// Test: Debug formatting present
/// What: Format the Error with `{:?}` and assert the output is non-empty.
/// Why: Guards against accidental removal of `Debug`.
#[test]
fn debug_format_non_empty_success() {
    let e = Error::new(Audience::User, Kind::ProcessingFailure, "proc");
    let s = format!("{:?}", e);
    assert!(!s.is_empty());
}

/// Test: empty message and extension of behavior
/// What: Construct with an empty message and ensure Display yields empty string and other accessors still work.
/// Why: Edge-case handling for empty messages should not break accessors or formatting.
#[test]
fn empty_message_is_allowed_success() {
    let e = Error::new(Audience::User, Kind::InvalidInput, "");
    assert_eq!(e.message, "");
    assert_eq!(format!("{}", e), "");
    assert!(e.is_user());
}