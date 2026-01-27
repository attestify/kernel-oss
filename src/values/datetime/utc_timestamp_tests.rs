use crate::error::Audience;
use crate::error::Kind;
use crate::values::datetime::utc_timestamp::UTCTimestamp;
use test_framework_oss::is_error;
use test_framework_oss::is_ok;
use test_framework_oss::kernel_error_eq;

#[test]
/// Verifies builder accepts a millisecond input, converts to nanoseconds, and that as_nano(),
/// as_milli(), and as_sec() return the expected fixed values.
fn from_millis_success() {
    let ms: u64 = 1_234_567;
    let ts = is_ok!(UTCTimestamp::builder().use_ms(ms).build());
    assert_eq!(ts.as_nano(), 1_234_567_000_000u128);
    assert_eq!(ts.as_milli(), 1_234_567u64);
    assert_eq!(ts.as_sec(), 1_234u64);
}

#[test]
/// Verifies builder accepts a nanosecond input and that as_nano(), as_milli(), and as_sec()
/// return the expected fixed values derived from the provided nanoseconds.
fn from_nanos_success() {
    let ns: u128 = 9_223_372_036_854_775_807u128;
    let ts = is_ok!(UTCTimestamp::builder().use_ns(ns).build());
    assert_eq!(ts.as_nano(), 9_223_372_036_854_775_807u128);
    assert_eq!(ts.as_milli(), 9_223_372_036_854u64);
    assert_eq!(ts.as_sec(), 9_223_372_036u64);
}

#[test]
/// Ensures builder call order is respected: a subsequent use_ns() overrides a prior use_ms(),
/// so the final timestamp reflects the last setter call.
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
/// Ensures using [u64::MAX] milliseconds produces the correct nanosecond value, preserves the
/// millisecond value, and produces the expected seconds value (no overflow in millisecond path).
///
/// Confirms the builder and accessors correctly handle the largest possible millisecond input
/// without arithmetic overflow when converting to nanoseconds or seconds. This protects against
/// silent integer overflow or incorrect rounding that could corrupt timestamps stored or sent to
/// other systems (logs, databases, protocols).
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
/// Confirms a very large nanosecond value causes as_milli() to cap at u64::MAX while as_nano()
/// returns the original large value and as_sec() computes the expected seconds.
///
/// Verifies a huge nanosecond input clamps the millisecond view to [u64::MAX] while still
/// preserving the original nanosecond value. This ensures the API documents and enforces a
/// safe millisecond range instead of producing a wrapped or unpredictable value, which is
/// important when callers expect a bounded millisecond representation.
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
/// Verifies that an extremely large nanosecond input results in as_milli() being capped at
/// u64::MAX and as_sec() wrapping/truncating to 0 due to the u64 cast behavior.\
///
/// Demonstrates the behavior when seconds computed from nanoseconds exceed u64 range (cast/wrap).
/// Explicitly testing this documents the implementation behavior (wrap/truncate) so callers know
/// the failure mode and avoid relying on undefined-looking values. It also prevents regressions
/// where a later change might silently panic or produce a different wrap/overflow behavior
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
/// Checks that sub-millisecond nanosecond values produce zero for both as_milli() and as_sec()
/// while as_nano() returns the original nanoseconds.
///
/// Ensures sub-millisecond inputs produce zero for as_milli() and as_sec() rather than rounding up.
/// This is important for correctness when low-resolution views are expected to floor the time
/// (e.g., time-based bucketing, TTL calculations) and prevents subtle off-by-one or
/// premature-expiry bugs.
fn small_ns_yields_zero_millis_and_seconds() {
    let ts = is_ok!(UTCTimestamp::builder().use_ns(999_999u128).build());
    assert_eq!(ts.as_nano(), 999_999u128);
    assert_eq!(ts.as_milli(), 0u64);
    assert_eq!(ts.as_sec(), 0u64);
}

#[test]
/// Validates integer-division truncation: as_milli() and as_sec() should floor the nanosecond
/// input (no rounding), and as_nano() remains unchanged.
///
/// Validates that integer division truncates toward zero (floors) for millisecond and second
/// conversions and does not perform rounding. This guarantees predictable, consistent
/// down-sampling behavior across the codebase and avoids surprising results in time comparisons,
/// rate calculations, or aggregated metrics.
fn non_multiple_truncation_behavior_success() {
    let ts = is_ok!(UTCTimestamp::builder().use_ns(1_234_567_890u128).build());
    assert_eq!(ts.as_nano(), 1_234_567_890u128);
    assert_eq!(ts.as_milli(), 1234u64); // floor(1_234_567_890 / 1_000_000)
    assert_eq!(ts.as_sec(), 1u64); // floor(1_234_567_890 / 1_000_000_000)
}

#[test]
/// Confirms building without setting any value returns an error with Kind::InvalidInput and the
/// expected system-audience error message.
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
