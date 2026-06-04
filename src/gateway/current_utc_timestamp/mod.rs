#[cfg(test)]
mod tests;

use crate::gateway::VoidGateway;
use crate::values::datetime::utc_timestamp::UTCTimestamp;

/// Defines the domain seam for retrieving the current UTC timestamp.
pub trait CurrentUTCTimestampGW: VoidGateway<Response = UTCTimestamp> {}
