#[cfg(test)]
mod tests;

use crate::gateway::Gateway;
use crate::ulid::ULID;

/// Void-by-construction request passed into the new identity gateway seam.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NewIdentityGatewayRequest;

/// Defines the domain seam for generating a new identity.
pub trait NewIdentityGW: Gateway<Request = NewIdentityGatewayRequest, Response = ULID> {}
