use crate::values::specification::api_version::APIVersion;
use crate::values::specification::kind::Kind;
use std::any::Any;
use std::fmt;

pub trait AssuranceProcedure: Any + fmt::Debug {
    fn api_version(&self) -> APIVersion;
    fn kind(&self) -> Kind {
        Kind::AssuranceProcedure
    }
    fn as_any(&self) -> &dyn Any;
}

pub trait AssuranceReport: Any + fmt::Debug {
    fn api_version(&self) -> APIVersion;
    fn kind(&self) -> Kind {
        Kind::AssuranceReport
    }
    fn as_any(&self) -> &dyn Any;
}
