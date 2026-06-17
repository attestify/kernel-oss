use crate::values::specification::api_version::APIVersion;
use crate::values::specification::kind::Kind;
use std::any::Any;
use std::fmt;

/// Common behavior for assurance procedure value objects.
pub trait AssuranceProcedure: Any + fmt::Debug {
    /// Returns the API version for this assurance procedure.
    fn api_version(&self) -> APIVersion;
    /// Returns the assurance procedure kind.
    fn kind(&self) -> Kind {
        Kind::AssuranceProcedure
    }
    /// Returns this value as [`Any`].
    fn as_any(&self) -> &dyn Any;
}

/// Common behavior for assurance report value objects.
pub trait AssuranceReport: Any + fmt::Debug {
    /// Returns the API version for this assurance report.
    fn api_version(&self) -> APIVersion;
    /// Returns the assurance report kind.
    fn kind(&self) -> Kind {
        Kind::AssuranceReport
    }
    /// Returns this value as [`Any`].
    fn as_any(&self) -> &dyn Any;
}
