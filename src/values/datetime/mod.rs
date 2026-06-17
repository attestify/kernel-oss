//! Date and time bounded values used by the kernel.

/// Start-time value helpers.
pub mod start_time;
pub mod utc_timestamp;

#[cfg(test)]
mod start_time_tests;

#[cfg(test)]
mod utc_timestamp_tests;
