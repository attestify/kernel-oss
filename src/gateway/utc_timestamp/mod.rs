use crate::error::Error;
use crate::values::datetime::utc_timestamp::UTCTimestamp;

#[deprecated(
    note = "Use gateway::current_utc_timestamp::CurrentUTCTimestampGW, which implements the shared Gateway seam."
)]
pub trait UTCTimestampGateway: UTCTimestampGatewayClone + Sync + Send {
    fn now(&self) -> Result<UTCTimestamp, Error>;
}

#[allow(deprecated)]
pub trait UTCTimestampGatewayClone {
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
