//! Shared use case role traits.
//!
//! Use [`VoidUseCase`] / [`AsyncVoidUseCase`] for true no-input use cases. Use
//! [`UseCase`] / [`AsyncUseCase`] when a finalized request crosses the use case
//! boundary. Domain-specific `*UC` traits should stay as thin marker
//! supertraits over these shared roles.

#[cfg(test)]
mod tests;

use crate::error::Error;
use crate::response::ResponseFuture;

/// Defines the shared synchronous execution role for a no-input use case seam.
pub trait VoidUseCase: Send + Sync {
    /// Successful use case response.
    type Response;

    /// Executes the use case without a request object.
    fn execute(&self) -> Result<Self::Response, Error>;
}

/// Defines the shared synchronous execution role for a use case seam.
pub trait UseCase: Send + Sync {
    /// Verified request accepted at the public use case boundary.
    type Request;

    /// Successful use case response.
    type Response;

    /// Executes the use case with the provided request.
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error>;
}

/// Defines the shared asynchronous execution role for a no-input use case seam.
pub trait AsyncVoidUseCase: Send + Sync {
    /// Successful use case response.
    type Response;

    /// Executes the use case and returns an explicit response future.
    fn execute<'a>(&'a self) -> ResponseFuture<'a, Self::Response>;
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
