//! Standards-aligned current UTC timestamp gateway.
//!
//! This module provides the shared kernel seam for retrieving the current UTC
//! timestamp, plus the sync and async marker traits that bind the shared
//! gateway role to that capability.

#[cfg(test)]
mod tests;

use crate::gateway::{AsyncVoidGateway, VoidGateway};
use crate::values::datetime::utc_timestamp::UTCTimestamp;

/// Defines the domain seam for retrieving the current UTC timestamp.
pub trait CurrentUTCTimestampGW: VoidGateway<Response = UTCTimestamp> {}

/// Defines the asynchronous domain seam for retrieving the current UTC timestamp.
pub trait AsyncCurrentUTCTimestampGW: AsyncVoidGateway<Response = UTCTimestamp> {}
