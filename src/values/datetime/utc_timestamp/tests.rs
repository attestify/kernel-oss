//! Tests for `UTCTimestamp`, covering builder inputs, conversion behavior, and failure cases.
//!
//! Bounded unit under test: `UTCTimestamp`.
//! Public interfaces verified: the builder, `as_nano`, `as_milli`, `as_sec`, and error handling.
//! Logical paths covered: millisecond input, nanosecond input, setter override behavior, overflow
//! handling, truncation behavior, zero-input rejection, and sub-millisecond conversion.
//! Requirement validation points: standards-aligned timestamp conversion and validation behavior.

use crate::error::Audience;
use crate::error::Kind;
use crate::values::datetime::utc_timestamp::UTCTimestamp;
use test_framework_oss::is_error;
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_eq;

#[test]
/// Requirement validation: verifies millisecond input converts to the expected timestamp views.
fn from_millis_success() {
    let ms: u64 = 1_234_567;
    let ts = is_ok!(UTCTimestamp::builder().use_ms(ms).build());
    assert_eq!(ts.as_nano(), 1_234_567_000_000u128);
    assert_eq!(ts.as_milli(), 1_234_567u64);
    assert_eq!(ts.as_sec(), 1_234u64);
}

#[test]
/// Requirement validation: verifies nanosecond input converts to the expected timestamp views.
fn from_nanos_success() {
    let ns: u128 = 9_223_372_036_854_775_807u128;
    let ts = is_ok!(UTCTimestamp::builder().use_ns(ns).build());
    assert_eq!(ts.as_nano(), 9_223_372_036_854_775_807u128);
    assert_eq!(ts.as_milli(), 9_223_372_036_854u64);
    assert_eq!(ts.as_sec(), 9_223_372_036u64);
}

#[test]
/// Requirement validation: verifies the last setter call wins when multiple builder inputs are
/// provided.
fn builder_override_last_call_success() {
    let ts = is_ok!(
        UTCTimestamp::builder()
            .use_ms(1_234_567u64)
            .use_ns(9_223_372_036_854_775_807u128)
            .build()
    );
    assert_eq!(ts.as_nano(), 9_223_372_036_854_775_807u128);
    assert_eq!(ts.as_milli(), 9_223_372_036_854u64);
    assert_eq!(ts.as_sec(), 9_223_372_036u64);
}

#[test]
/// Requirement validation: verifies `u64::MAX` milliseconds convert without overflow.
fn ms_at_u64_max_success() {
    let ts = is_ok!(
        UTCTimestamp::builder()
            .use_ms(18446744073709551615u64)
            .build()
    );
    assert_eq!(ts.as_nano(), 18446744073709551615000000u128);
    assert_eq!(ts.as_milli(), 18446744073709551615u64);
    assert_eq!(ts.as_sec(), 18446744073709551u64);
}

#[test]
/// Requirement validation: verifies very large nanosecond inputs cap the millisecond view.
fn ns_triggers_millis_cap_success() {
    let ts = is_ok!(
        UTCTimestamp::builder()
            .use_ns(18446744073709551616000000u128) // (u64::MAX + 1) * 1_000_000
            .build()
    );
    assert_eq!(ts.as_nano(), 18446744073709551616000000u128);
    assert_eq!(ts.as_milli(), 18446744073709551615u64); // capped
    assert_eq!(ts.as_sec(), 18446744073709551u64);
}

#[test]
/// Requirement validation: verifies extremely large nanosecond inputs wrap the seconds view as
/// implemented.
fn ns_causes_seconds_wrap_to_zero_success() {
    let ts = is_ok!(
        UTCTimestamp::builder()
            .use_ns(18446744073709551616000000000u128) // (u64::MAX + 1) * 1_000_000_000
            .build()
    );
    assert_eq!(ts.as_nano(), 18446744073709551616000000000u128);
    assert_eq!(ts.as_milli(), 18446744073709551615u64); // capped
    assert_eq!(ts.as_sec(), 0u64); // cast wraps/truncates
}

#[test]
/// Requirement validation: verifies sub-millisecond inputs floor to zero for milli and seconds.
fn small_ns_yields_zero_millis_and_seconds_success() {
    let ts = is_ok!(UTCTimestamp::builder().use_ns(999_999u128).build());
    assert_eq!(ts.as_nano(), 999_999u128);
    assert_eq!(ts.as_milli(), 0u64);
    assert_eq!(ts.as_sec(), 0u64);
}

#[test]
/// Requirement validation: verifies integer division truncates rather than rounds.
fn non_multiple_truncation_behavior_success() {
    let ts = is_ok!(UTCTimestamp::builder().use_ns(1_234_567_890u128).build());
    assert_eq!(ts.as_nano(), 1_234_567_890u128);
    assert_eq!(ts.as_milli(), 1234u64); // floor(1_234_567_890 / 1_000_000)
    assert_eq!(ts.as_sec(), 1u64); // floor(1_234_567_890 / 1_000_000_000)
}

#[test]
/// Requirement validation: verifies missing inputs are rejected during build.
fn missing_value_error() {
    let datetime = UTCTimestamp::builder().build();
    is_error!(&datetime);
    kernel_error_eq!(
        &datetime,
        Kind::InvalidInput,
        Audience::System,
        "A value was not provided for the UTCTimestamp, please provide a valid UTCTimestamp value."
    );
}
