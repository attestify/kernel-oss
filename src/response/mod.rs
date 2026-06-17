use crate::error::Error;
use std::future::Future;
use std::pin::Pin;

/// A boxed future returned by asynchronous bounded seams.
pub type ResponseFuture<'a, Response> =
    Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>>;
