//! Tests for `StartTime`, covering construction, conversion, and validation behavior.
//!
//! Bounded unit under test: `StartTime`.
//! Public interfaces verified: `now`, `from`, `try_from`, and `to_string`.
//! Public interfaces verified: `time` accessor.
//! Logical paths covered: current time capture, explicit construction, successful parsing, and the
//! zero-value rejection path.
//! Requirement validation points: standards-aligned behavior for the start-time value object.

use crate::error::Audience;
use crate::error::Kind;
use crate::values::datetime::start_time::StartTime;
use test_framework_oss::{is_error, is_ok};

#[test]
/// Requirement validation: verifies `StartTime::now` returns a non-zero timestamp.
fn now_success() {
    let start_time = StartTime::now();
    assert_ne!(start_time.time, 0);
}

#[test]
/// Requirement validation: verifies `StartTime::from` preserves the provided timestamp.
fn from_success() {
    let start_time = StartTime::from(1000);
    assert_eq!(start_time.time, 1000);
}

#[test]
/// Requirement validation: verifies `StartTime::time` returns the stored timestamp.
fn time_accessor_success() {
    let start_time = StartTime::from(1000);
    assert_eq!(start_time.time(), 1000);
}

#[test]
/// Requirement validation: verifies `StartTime::try_from` accepts a positive timestamp.
fn try_from_success() {
    is_ok!(StartTime::try_from(1000));
}

#[test]
/// Requirement validation: verifies `StartTime::to_string` renders the stored timestamp.
fn to_string_success() {
    let start_time = StartTime::from(1000);
    assert_eq!(start_time.to_string(), "1000");
}

#[test]
/// Requirement validation: verifies `StartTime::try_from` rejects zero.
fn try_from_zero_error() {
    let result = StartTime::try_from(0);
    let error = is_error!(result);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(
        error.message,
        "The start time cannot be 0.  Please provide a valid utc time."
    );
}
