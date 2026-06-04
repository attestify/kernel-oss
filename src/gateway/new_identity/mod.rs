#[cfg(test)]
mod tests;

use crate::gateway::{AsyncVoidGateway, VoidGateway};
use crate::ulid::ULID;

/// Defines the domain seam for generating a new identity.
pub trait NewIdentityGW: VoidGateway<Response = ULID> {}

/// Defines the asynchronous domain seam for generating a new identity.
pub trait AsyncNewIdentityGW: AsyncVoidGateway<Response = ULID> {}
