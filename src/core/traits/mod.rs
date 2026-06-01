use crate::error::Error;
use std::future::Future;
use std::pin::Pin;

/// A boxed future returned by asynchronous bounded seams.
pub type ResponseFuture<'a, Response> =
    Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>>;

pub trait To<T> {
    fn to(&self) -> T;
}

/// Exposes the stable identity held by an entity.
pub trait Entity {
    /// The bounded identity type for the entity.
    type IdType;

    /// Returns the entity identity.
    fn id(&self) -> &Self::IdType;
}
