#[cfg(test)]
mod tests;

use crate::gateway::Gateway;
use crate::values::datetime::utc_timestamp::UTCTimestamp;

/// Void-by-construction request passed into the current UTC timestamp gateway seam.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CurrentUTCTimestampGatewayRequest;

/// Defines the domain seam for retrieving the current UTC timestamp.
pub trait CurrentUTCTimestampGW:
    Gateway<Request = CurrentUTCTimestampGatewayRequest, Response = UTCTimestamp>
{
}
