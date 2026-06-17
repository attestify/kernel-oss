//! Verifies the file-data gateway seam.
//!
//! Bounded unit under test:
//! - `FileDataRequest`
//! - `FileDataGW`
//! - `AsyncFileDataGW`
//! - `FileDataFnGateway`
//!
//! Public interfaces verified:
//! - `FileDataRequest::builder().try_build()`
//! - `Gateway::execute(&gateway as &dyn FileDataGW, request)`
//! - `AsyncGateway::execute(&gateway as &dyn AsyncFileDataGW, request)`
//!
//! Logical paths covered:
//! - request construction rejects missing and blank file paths
//! - synchronous marker-seam execution returns file bytes
//! - asynchronous marker-seam execution returns file bytes
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::{Error, Kind};
use crate::gateway::file_data::{AsyncFileDataGW, FileDataFnGateway, FileDataGW, FileDataRequest};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;
use crate::values::specification::file_path::FilePath;
use std::task::{Context, Poll};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the file path
/// is missing.
#[test]
fn request_builder_rejects_missing_file_path_error() {
    let result = FileDataRequest::builder().try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the file path
/// contains only whitespace.
#[test]
fn request_builder_rejects_whitespace_file_path_error() {
    let result = FileDataRequest::builder().file_path("   ").try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder accepts an already validated file path.
#[test]
fn request_builder_accepts_valid_file_path_success() {
    let file_path = FilePath::try_from("/tmp/evidence.txt")
        .expect("Expected file path fixture to build for file data request verification.");

    let result = FileDataRequest::builder()
        .valid_file_path(file_path.clone())
        .try_build();

    let request = is_ok!(result);
    assert_eq!(request.file_path(), &file_path);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the synchronous marker seam
/// returns file bytes.
#[test]
fn execute_through_marker_seam_returns_file_data_success() {
    let gateway = FileDataFnGateway::new(file_data_fixture);
    let request = request_fixture();

    let result = Gateway::execute(&gateway as &dyn FileDataGW, request);

    let data = is_ok!(result);
    assert_eq!(data, b"evidence".to_vec());
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the asynchronous marker seam
/// returns file bytes.
#[test]
fn async_execute_through_marker_seam_returns_file_data_success() {
    let gateway = FileDataFnGateway::new(file_data_fixture);
    let request = request_fixture();

    let result = try_run_ready(AsyncGateway::execute(
        &gateway as &dyn AsyncFileDataGW,
        request,
    ));

    let data = is_ok!(result);
    assert_eq!(data, b"evidence".to_vec());
}

fn request_fixture() -> FileDataRequest {
    FileDataRequest::builder()
        .file_path("/tmp/evidence.txt")
        .try_build()
        .expect("Expected file data request fixture to build.")
}

fn file_data_fixture(file_path: &str) -> Result<Vec<u8>, Error> {
    if file_path == "/tmp/evidence.txt" {
        return Ok(b"evidence".to_vec());
    }

    Err(Error::for_system(
        Kind::NotFound,
        "The requested file data was not found.",
    ))
}

fn try_run_ready<Response>(mut future: ResponseFuture<'_, Response>) -> Result<Response, Error> {
    let mut context = Context::from_waker(std::task::Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("Expected gateway response future to be ready for verification."),
    }
}
