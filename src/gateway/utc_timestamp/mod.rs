use crate::error::Error;
use crate::values::datetime::utc_timestamp::UTCTimestamp;

pub trait UTCTimestampGateway: UTCTimestampGatewayClone + Sync + Send {
    fn now(&self) -> Result<UTCTimestamp, Error>;
}

pub trait UTCTimestampGatewayClone {
    fn clone_box(&self) -> Box<dyn UTCTimestampGateway>;
}

impl<Gw> UTCTimestampGatewayClone for Gw
where
    Gw: 'static + UTCTimestampGateway + Clone,
{
    fn clone_box(&self) -> Box<dyn UTCTimestampGateway> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UTCTimestampGateway> {
    fn clone(&self) -> Box<dyn UTCTimestampGateway> {
        self.clone_box()
    }
}
