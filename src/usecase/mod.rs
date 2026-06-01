#[cfg(test)]
mod tests;

use crate::core::traits::ResponseFuture;
use crate::error::Error;

/// Defines the shared synchronous execution role for a use case seam.
pub trait UseCase: Send + Sync {
    /// Verified request accepted at the public use case boundary.
    type Request;

    /// Successful use case response.
    type Response;

    /// Executes the use case with the provided request.
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error>;
}

/// Defines the shared asynchronous execution role for a use case seam.
pub trait AsyncUseCase: Send + Sync {
    /// Verified request accepted at the public use case boundary.
    type Request;

    /// Successful use case response.
    type Response;

    /// Executes the use case and returns an explicit response future.
    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response>;
}
