pub mod directory_list;
pub mod file_data_gateway;
pub mod identity;
pub mod logger;

pub mod utc_timestamp;

use std::pin::Pin;
use crate::error::Error;

/// Defines the shared synchronous execution role for a gateway seam.
pub trait Gateway: Send + Sync {
    /// Builder accepted at the public gateway boundary.
    type RequestBuilder;

    /// Verified request produced by the request builder.
    type Request;

    /// Successful gateway response.
    type Response;

    /// Executes the gateway with the provided request builder.
    fn execute(&self, request_builder: Self::RequestBuilder) -> Result<Self::Response, Error>;
}

/// Defines the shared asynchronous execution role for a gateway seam.
pub trait AsyncGateway: Send + Sync {
    /// Builder accepted at the public gateway boundary.
    type RequestBuilder;

    /// Verified request produced by the request builder.
    type Request;

    /// Successful gateway response.
    type Response;

    /// Executes the gateway and returns an explicit response future.
    fn execute<'a>(
        &'a self,
        request_builder: Self::RequestBuilder,
    ) -> GWResponseFuture<'a, Self::Response>;
}

/// A boxed future returned by asynchronous UsecCase seams.
pub type GWResponseFuture<'a, GWResponse> =
Pin<Box<dyn Future<Output = Result<GWResponse, Error>> + Send + 'a>>;
