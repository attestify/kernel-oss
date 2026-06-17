//! Legacy UTC timestamp gateway compatibility surface.
//!
//! Prefer [`crate::gateway::current_utc_timestamp::CurrentUTCTimestampGW`] and
//! [`crate::gateway::current_utc_timestamp::AsyncCurrentUTCTimestampGW`] in new code.

use crate::error::Error;
use crate::values::datetime::utc_timestamp::UTCTimestamp;

#[deprecated(
    note = "Use gateway::current_utc_timestamp::CurrentUTCTimestampGW, which implements the shared Gateway seam."
)]
/// A legacy UTC timestamp gateway trait kept for compatibility.
pub trait UTCTimestampGateway: UTCTimestampGatewayClone + Sync + Send {
    /// Returns the current UTC timestamp.
    fn now(&self) -> Result<UTCTimestamp, Error>;
}

#[allow(deprecated)]
/// Clone support for boxed UTC timestamp gateways.
pub trait UTCTimestampGatewayClone {
    /// Clones the gateway trait object into a boxed trait object.
    fn clone_box(&self) -> Box<dyn UTCTimestampGateway>;
}

#[allow(deprecated)]
impl<Gw> UTCTimestampGatewayClone for Gw
where
    Gw: 'static + UTCTimestampGateway + Clone,
{
    fn clone_box(&self) -> Box<dyn UTCTimestampGateway> {
        Box::new(self.clone())
    }
}

#[allow(deprecated)]
impl Clone for Box<dyn UTCTimestampGateway> {
    fn clone(&self) -> Box<dyn UTCTimestampGateway> {
        self.clone_box()
    }
}
