pub mod current_utc_timestamp;
pub mod directory_list;
pub mod file_data_gateway;
pub mod identity;
pub mod logger;
pub mod new_identity;

#[cfg(test)]
mod tests;
pub mod utc_timestamp;

use crate::core::traits::ResponseFuture;
use crate::error::Error;

/// Defines the shared synchronous execution role for a gateway seam.
pub trait Gateway: Send + Sync {
    /// Verified request accepted at the public gateway boundary.
    type Request;

    /// Successful gateway response.
    type Response;

    /// Executes the gateway with the provided request.
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error>;
}

/// Defines the shared asynchronous execution role for a gateway seam.
pub trait AsyncGateway: Send + Sync {
    /// Verified request accepted at the public gateway boundary.
    type Request;

    /// Successful gateway response.
    type Response;

    /// Executes the gateway and returns an explicit response future.
    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response>;
}
