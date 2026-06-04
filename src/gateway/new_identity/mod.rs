#[cfg(test)]
mod tests;

use crate::gateway::VoidGateway;
use crate::ulid::ULID;

/// Defines the domain seam for generating a new identity.
pub trait NewIdentityGW: VoidGateway<Response = ULID> {}
