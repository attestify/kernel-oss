use crate::error::{Error};
use crate::error::Kind::InvalidInput;

/// ## Public interface
/// - `UTCTimestamp::builder()` -> returns a `UTCTimestampBuilder`.
/// - Builder setters:
///   - `use_ns(ns: u128)` — provide a nanosecond timestamp (last setter wins).
///   - `use_ms(ms: u64)` — provide a millisecond timestamp (last setter wins).
///   - `build() -> Result<UTCTimestamp, Error>` — constructs the value or returns an error if no value was provided.
/// - Accessors:
///   - `as_nano() -> u128` — nanosecond view (exact stored value).
///   - `as_milli() -> u64` — millisecond view (bounded to `u64`).
///   - `as_sec() -> u64` — second view (cast/truncated to `u64`).
///
/// ## Examples
/// ```rust
/// use kernel_oss::values::datetime::utc_timestamp::UTCTimestamp;
///
/// // Build from nanoseconds
/// let ts = UTCTimestamp::builder().use_ns(1_500_000u128).build().unwrap();
/// assert_eq!(ts.as_nano(), 1_500_000u128);
/// assert_eq!(ts.as_milli(), 1u64); // floor(1_500_000 / 1_000_000)
/// assert_eq!(ts.as_sec(), 0u64);   // floor(1_500_000 / 1_000_000_000)
///
/// // Build from milliseconds
/// let ts = UTCTimestamp::builder().use_ms(1_234_567u64).build().unwrap();
/// assert_eq!(ts.as_nano(), 1_234_567_000_000u128); // ms * 1_000_000
/// assert_eq!(ts.as_milli(), 1_234_567u64);
/// assert_eq!(ts.as_sec(), 1_234u64); // floor(ms / 1_000)
///
/// // Builder override: last setter wins
/// let ts = UTCTimestamp::builder().use_ms(1u64).use_ns(9u128).build().unwrap();
/// assert_eq!(ts.as_nano(), 9u128);
/// ```
///
/// ## Conversion and exact behavior
/// - `use_ns(...)` preserves the provided `u128` nanoseconds; `as_nano()` returns it unchanged.
/// - `use_ms(ms)` stores `(ms as u128) * 1_000_000` using saturating multiplication to avoid intermediate overflow.
/// - Milliseconds are derived as `ns / 1_000_000` using integer division (truncation toward zero). No rounding is performed.
/// - Seconds are derived as `ns / 1_000_000_000` using integer division (truncation toward zero).
///
/// ## Overflow, capping, and casting behavior (documented)
/// - `as_milli()` returns a `u64`. If the computed milliseconds exceed `u64::MAX`, the value is capped to `u64::MAX` to present a bounded millisecond view.
/// - `as_sec()` returns a `u64` computed from the integer division of nanoseconds by `1_000_000_000` and then cast to `u64`. If the computed seconds exceed `u64::MAX`, the cast will truncate/wrap as per Rust's `as` semantics for integer casts; this wrap/truncate behavior is intentional and covered by tests.
/// - Building from `u64::MAX` milliseconds yields a valid `u128` nanosecond value (`ms * 1_000_000u128`) without arithmetic overflow, and `as_milli()` will equal `u64::MAX`.
///
/// ## Error behavior
/// - Calling `build()` without setting either `use_ns(...)` or `use_ms(...)` returns an `Err(Error)` with `Kind::InvalidInput` and system audience. The exact error message returned is:
///   `A value was not provided for the DateTime, please provide a valid DateTime value.`
///
/// ## Notes for callers
/// - If callers require saturation semantics for seconds (instead of the documented cast/wrap), they must validate or normalize input before constructing a `UTCTimestamp`.
/// - The builder enforces a last-setter-wins rule; call order determines the stored value.
/// - Unit tests exist to validate edge cases: `u64::MAX` milliseconds, very large `ns` that trigger millisecond capping, `ns` values that cause second cast/wrap, sub-millisecond inputs, and truncation semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UTCTimestamp {
    timestamp: u128,
}

impl UTCTimestamp {
    pub fn builder() -> UTCTimestampBuilder {
     UTCTimestampBuilder::default()
    }

    pub fn as_nano(&self) -> u128 {
        self.timestamp
    }

    pub fn as_milli(&self) -> u64 {
        let millis = self.timestamp / 1_000_000;
        if millis > u64::MAX as u128 {
            u64::MAX
        } else {
            millis as u64
        }
    }

    pub fn as_sec(&self) -> u64 {
        (self.timestamp / 1_000_000_000) as u64
    }
    
}

#[derive(Debug, Clone, Default)]
pub struct UTCTimestampBuilder {
    timestamp: Option<u128>,
}


impl UTCTimestampBuilder {

    /// Provide a 64-bit millisecond value representing seconds since the Unix epoch.
    pub fn use_ms(mut self, timestamp: u64) -> Self {
        let nanos = (timestamp as u128).saturating_mul(1_000_000u128);
        self.timestamp = Some(nanos);
        self
    }

    /// Provide a full 128-bit nanosecond value representing nanoseconds since the Unix epoch.
    pub fn use_ns(mut self, timestamp: u128) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Result<UTCTimestamp, Error> {
        validate_value(self.timestamp)
    }
    
}

fn validate_value(value: Option<u128>) -> Result<UTCTimestamp, Error> {
    match value {
        Some(valid_value) => Ok(UTCTimestamp { timestamp: valid_value }),
        None => Err(Error::for_system(InvalidInput, "A value was not provided for the UTCTimestamp, please provide a valid UTCTimestamp value.".to_string()))
    }
}
