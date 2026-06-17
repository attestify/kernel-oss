//! Verifies the retrieve-directory-path gateway seam.
//!
//! Bounded unit under test:
//! - `RetrieveDirectoryPathRequest`
//! - `RetrieveDirectoryPathGW`
//! - `AsyncRetrieveDirectoryPathGW`
//! - `RetrieveDirectoryPathFnGateway`
//!
//! Public interfaces verified:
//! - `RetrieveDirectoryPathRequest::builder().try_build()`
//! - `Gateway::execute(&gateway as &dyn RetrieveDirectoryPathGW, request)`
//! - `AsyncGateway::execute(&gateway as &dyn AsyncRetrieveDirectoryPathGW, request)`
//!
//! Logical paths covered:
//! - request construction rejects missing and blank keys
//! - synchronous marker-seam execution returns the resolved directory path
//! - asynchronous marker-seam execution returns the resolved directory path
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::{Error, Kind};
use crate::gateway::retrieve_directory_path::{
    AsyncRetrieveDirectoryPathGW, RetrieveDirectoryPathFnGateway, RetrieveDirectoryPathGW,
    RetrieveDirectoryPathRequest,
};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;
use std::task::{Context, Poll};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the directory
/// key is missing.
#[test]
fn request_builder_rejects_missing_directory_key_error() {
    let result = RetrieveDirectoryPathRequest::builder().try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the directory
/// key contains only whitespace.
#[test]
fn request_builder_rejects_whitespace_directory_key_error() {
    let result = RetrieveDirectoryPathRequest::builder()
        .directory_key("   ")
        .try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the synchronous marker seam
/// returns the resolved directory path.
#[test]
fn execute_through_marker_seam_returns_directory_path_success() {
    let gateway = RetrieveDirectoryPathFnGateway::new(retrieve_directory_path_fixture);
    let request = request_fixture();

    let result = Gateway::execute(&gateway as &dyn RetrieveDirectoryPathGW, request);

    let directory_path = is_ok!(result);
    assert_eq!(directory_path, "/tmp/evidence");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the asynchronous marker seam
/// returns the resolved directory path.
#[test]
fn async_execute_through_marker_seam_returns_directory_path_success() {
    let gateway = RetrieveDirectoryPathFnGateway::new(retrieve_directory_path_fixture);
    let request = request_fixture();

    let result = try_run_ready(AsyncGateway::execute(
        &gateway as &dyn AsyncRetrieveDirectoryPathGW,
        request,
    ));

    let directory_path = is_ok!(result);
    assert_eq!(directory_path, "/tmp/evidence");
}

fn request_fixture() -> RetrieveDirectoryPathRequest {
    RetrieveDirectoryPathRequest::builder()
        .directory_key("evidence")
        .try_build()
        .expect("Expected retrieve directory path request fixture to build.")
}

fn retrieve_directory_path_fixture(directory_key: &str) -> Result<String, Error> {
    if directory_key == "evidence" {
        return Ok("/tmp/evidence".to_string());
    }

    Err(Error::for_system(
        Kind::NotFound,
        "The requested directory path was not found.",
    ))
}

fn try_run_ready<Response>(mut future: ResponseFuture<'_, Response>) -> Result<Response, Error> {
    let mut context = Context::from_waker(std::task::Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("Expected gateway response future to be ready for verification."),
    }
}
