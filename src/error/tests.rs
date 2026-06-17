//! Verifies the bounded kernel error type.
//!
//! Bounded unit under test:
//! - `Error`
//!
//! Public interfaces verified:
//! - `Error::new`
//! - `Error::for_user`
//! - `Error::for_system`
//! - `Display` and equality/hash behavior
//!
//! Logical paths covered:
//! - error construction stores audience, kind, and message
//! - convenience constructors set the expected audience
//! - clone, equality, and hash remain consistent
//! - hash-based lookup accepts equal errors
//! - debug formatting remains available
//! - empty error messages remain representable
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{Audience, Error, Kind};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that `Error::new` stores the supplied audience, kind, and message
/// and keeps its display output aligned with the message.
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

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the convenience constructors set the correct audience for
/// user and system errors.
#[test]
fn convenience_constructors_set_audience_success() {
    let u = Error::for_user(Kind::NotFound, "not found");
    let s = Error::for_system(Kind::Unexpected, "oh no");
    assert!(u.is_user() && !u.is_system());
    assert!(s.is_system() && !s.is_user());
    assert_eq!(u.kind, Kind::NotFound);
    assert_eq!(s.kind, Kind::Unexpected);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that cloning an `Error` preserves equality and hash behavior.
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

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that equal errors can be stored and found in a hash-based set.
#[test]
fn hashset_contains_equal_success() {
    let a = Error::for_user(Kind::PermissionDenied, "denied");
    let b = Error::for_user(Kind::PermissionDenied, "denied");
    let mut set: HashSet<Error> = HashSet::new();
    set.insert(a.clone());
    assert!(set.contains(&b));
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the debug representation remains available and non-empty.
#[test]
fn debug_format_non_empty_success() {
    let e = Error::new(Audience::User, Kind::ProcessingFailure, "proc");
    let s = format!("{:?}", e);
    assert!(!s.is_empty());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty messages remain representable without breaking accessors
/// or display behavior.
#[test]
fn empty_message_is_allowed_success() {
    let e = Error::new(Audience::User, Kind::InvalidInput, "");
    assert_eq!(e.message, "");
    assert_eq!(format!("{}", e), "");
    assert!(e.is_user());
}
