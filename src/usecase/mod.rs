use std::pin::Pin;
use crate::error::Error;

/// Defines the shared synchronous execution role for a use case seam.
pub trait UseCase: Send + Sync {
	/// Builder accepted at the public use case boundary.
	type RequestBuilder;

	/// Verified request produced by the request builder.
	type Request;

	/// Successful use case response.
	type Response;

	/// Executes the use case with the provided request builder.
	fn execute(&self, request_builder: Self::RequestBuilder) -> Result<Self::Response, Error>;
}

/// Defines the shared asynchronous execution role for a use case seam.
pub trait AsyncUseCase: Send + Sync {
	/// Builder accepted at the public use case boundary.
	type RequestBuilder;

	/// Verified request produced by the request builder.
	type Request;

	/// Successful use case response.
	type Response;

	/// Executes the use case and returns an explicit response future.
	fn execute<'a>(
		&'a self,
		request_builder: Self::RequestBuilder,
	) -> UCResponseFuture<'a, Self::Response>;
}


/// A boxed future returned by asynchronous UsecCase seams.
pub type UCResponseFuture<'a, UCResponse> =
Pin<Box<dyn Future<Output = Result<UCResponse, Error>> + Send + 'a>>;
