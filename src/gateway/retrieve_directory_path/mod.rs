//! Standards-aligned retrieve-directory-path gateway.
//!
//! This module provides the request-bearing gateway seam for resolving a
//! directory path from a directory key, along with the compatibility adapter
//! for the legacy function-type API.

#[cfg(test)]
mod tests;

use crate::error::{Error, Kind};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;
use crate::values::Value;

/// Built request for retrieving a directory path by key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RetrieveDirectoryPathRequest {
    directory_key: String,
}

impl RetrieveDirectoryPathRequest {
    /// Starts construction for a [`RetrieveDirectoryPathRequest`].
    pub fn builder() -> RetrieveDirectoryPathRequestBuilder {
        RetrieveDirectoryPathRequestBuilder::default()
    }

    /// Returns the validated directory key.
    pub fn directory_key(&self) -> &str {
        &self.directory_key
    }
}

impl Value for RetrieveDirectoryPathRequest {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.directory_key()
    }
}

/// Builds a [`RetrieveDirectoryPathRequest`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RetrieveDirectoryPathRequestBuilder {
    directory_key: Option<String>,
}

impl RetrieveDirectoryPathRequestBuilder {
    /// Sets the directory key to resolve.
    pub fn directory_key(mut self, directory_key: impl Into<String>) -> Self {
        self.directory_key = Some(directory_key.into());
        self
    }

    /// Validates and returns a built [`RetrieveDirectoryPathRequest`].
    pub fn try_build(mut self) -> Result<RetrieveDirectoryPathRequest, Error> {
        let directory_key = self.validate_directory_key()?;

        Ok(RetrieveDirectoryPathRequest { directory_key })
    }

    fn validate_directory_key(&mut self) -> Result<String, Error> {
        let directory_key = self.directory_key.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A retrieve directory path key is required, but none was provided.",
            )
        })?;

        if directory_key.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The retrieve directory path key provided is empty, provide a non-empty retrieve directory path key.",
            ));
        }

        if directory_key.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The retrieve directory path key provided contains only whitespace, provide a non-empty retrieve directory path key.",
            ));
        }

        Ok(directory_key)
    }
}

/// Defines the domain seam for retrieving a directory path by key.
pub trait RetrieveDirectoryPathGW:
    Gateway<Request = RetrieveDirectoryPathRequest, Response = String>
{
}

/// Defines the asynchronous domain seam for retrieving a directory path by key.
pub trait AsyncRetrieveDirectoryPathGW:
    AsyncGateway<Request = RetrieveDirectoryPathRequest, Response = String>
{
}

/// Adapts a legacy retrieve-directory-path function to the shared gateway seam.
#[derive(Clone, Copy)]
pub struct RetrieveDirectoryPathFnGateway {
    retrieve_directory_path: fn(directory_key: &str) -> Result<String, Error>,
}

impl RetrieveDirectoryPathFnGateway {
    /// Creates a gateway adapter from a legacy retrieve-directory-path function.
    pub fn new(retrieve_directory_path: fn(directory_key: &str) -> Result<String, Error>) -> Self {
        Self {
            retrieve_directory_path,
        }
    }
}

impl Gateway for RetrieveDirectoryPathFnGateway {
    type Request = RetrieveDirectoryPathRequest;
    type Response = String;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        (self.retrieve_directory_path)(request.directory_key())
    }
}

impl RetrieveDirectoryPathGW for RetrieveDirectoryPathFnGateway {}

impl AsyncGateway for RetrieveDirectoryPathFnGateway {
    type Request = RetrieveDirectoryPathRequest;
    type Response = String;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move { Gateway::execute(self, request) })
    }
}

impl AsyncRetrieveDirectoryPathGW for RetrieveDirectoryPathFnGateway {}
