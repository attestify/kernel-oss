#[cfg(test)]
mod tests;

use crate::error::{Error, Kind};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;
use crate::values::Value;
use crate::values::specification::file_path::FilePath;

/// Built request for retrieving file data from a file path.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileDataRequest {
    file_path: FilePath,
}

impl FileDataRequest {
    /// Starts construction for a [`FileDataRequest`].
    pub fn builder() -> FileDataRequestBuilder {
        FileDataRequestBuilder::default()
    }

    /// Returns the validated file path.
    pub fn file_path(&self) -> &FilePath {
        &self.file_path
    }
}

/// Builds a [`FileDataRequest`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileDataRequestBuilder {
    file_path: Option<FileDataRequestPath>,
}

impl FileDataRequestBuilder {
    /// Sets the raw file path text for the request.
    pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
        self.file_path = Some(FileDataRequestPath::Raw(file_path.into()));
        self
    }

    /// Sets an already validated file path for the request.
    pub fn valid_file_path(mut self, file_path: FilePath) -> Self {
        self.file_path = Some(FileDataRequestPath::Valid(file_path));
        self
    }

    /// Validates and returns a built [`FileDataRequest`].
    pub fn try_build(mut self) -> Result<FileDataRequest, Error> {
        let file_path = self.validate_file_path()?;

        Ok(FileDataRequest { file_path })
    }

    fn validate_file_path(&mut self) -> Result<FilePath, Error> {
        let file_path = self.file_path.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A file data path is required, but none was provided.",
            )
        })?;

        match file_path {
            FileDataRequestPath::Raw(path) => {
                if path.is_empty() {
                    return Err(Error::for_user(
                        Kind::InvalidInput,
                        "The file data path provided is empty, provide a non-empty file data path.",
                    ));
                }

                if path.trim().is_empty() {
                    return Err(Error::for_user(
                        Kind::InvalidInput,
                        "The file data path provided contains only whitespace, provide a non-empty file data path.",
                    ));
                }

                FilePath::try_from(&path)
            }
            FileDataRequestPath::Valid(path) => Ok(path),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum FileDataRequestPath {
    Raw(String),
    Valid(FilePath),
}

/// Defines the domain seam for retrieving file data.
pub trait FileDataGW: Gateway<Request = FileDataRequest, Response = Vec<u8>> {}

/// Defines the asynchronous domain seam for retrieving file data.
pub trait AsyncFileDataGW: AsyncGateway<Request = FileDataRequest, Response = Vec<u8>> {}

/// Adapts a legacy file-data function to the shared gateway seam.
#[derive(Clone, Copy)]
pub struct FileDataFnGateway {
    file_data_gateway: fn(file_path: &str) -> Result<Vec<u8>, Error>,
}

impl FileDataFnGateway {
    /// Creates a gateway adapter from a legacy file-data function.
    pub fn new(file_data_gateway: fn(file_path: &str) -> Result<Vec<u8>, Error>) -> Self {
        Self { file_data_gateway }
    }
}

impl Gateway for FileDataFnGateway {
    type Request = FileDataRequest;
    type Response = Vec<u8>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        (self.file_data_gateway)(request.file_path().value())
    }
}

impl FileDataGW for FileDataFnGateway {}

impl AsyncGateway for FileDataFnGateway {
    type Request = FileDataRequest;
    type Response = Vec<u8>;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move { Gateway::execute(self, request) })
    }
}

impl AsyncFileDataGW for FileDataFnGateway {}
