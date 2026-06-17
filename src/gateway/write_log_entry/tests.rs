//! Verifies the write-log-entry gateway seam.
//!
//! Bounded unit under test:
//! - `LogLevel`
//! - `WriteLogEntryRequest`
//! - `WriteLogEntryGW`
//! - `AsyncWriteLogEntryGW`
//! - `WriteLogEntryFnGateway`
//!
//! Public interfaces verified:
//! - `WriteLogEntryRequest::builder().try_build()`
//! - `Gateway::execute(&gateway as &dyn WriteLogEntryGW, request)`
//! - `AsyncGateway::execute(&gateway as &dyn AsyncWriteLogEntryGW, request)`
//!
//! Logical paths covered:
//! - request construction rejects missing levels
//! - request construction rejects missing and blank messages
//! - request construction rejects blank context
//! - request construction preserves the distinction between message and error
//! - synchronous marker-seam execution writes one log entry
//! - asynchronous marker-seam execution writes one log entry
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::{Error, Kind};
use crate::gateway::write_log_entry::{
    AsyncWriteLogEntryGW, LogLevel, WriteLogEntryFnGateway, WriteLogEntryGW, WriteLogEntryRequest,
};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;
use std::task::{Context, Poll};
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the log level
/// is missing.
#[test]
fn request_builder_rejects_missing_level_error() {
    let result = WriteLogEntryRequest::builder()
        .message("Application started.")
        .try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the log
/// message is missing.
#[test]
fn request_builder_rejects_missing_message_error() {
    let result = WriteLogEntryRequest::builder()
        .level(LogLevel::Info)
        .try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when the log
/// message contains only whitespace.
#[test]
fn request_builder_rejects_whitespace_message_error() {
    let result = WriteLogEntryRequest::builder()
        .level(LogLevel::Info)
        .message("   ")
        .try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the request builder returns a bounded error when optional
/// context is supplied but blank.
#[test]
fn request_builder_rejects_whitespace_context_error() {
    let result = WriteLogEntryRequest::builder()
        .level(LogLevel::Info)
        .message("Application started.")
        .context("   ")
        .try_build();

    let error = is_error!(result);
    assert_eq!(error.kind(), Kind::InvalidInput);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a log entry can carry both primary event text and optional
/// structured error context without collapsing one into the other.
#[test]
fn request_builder_preserves_message_and_error_success() {
    let expected_error = Error::for_system(Kind::GatewayError, "The provider failed.");

    let result = WriteLogEntryRequest::builder()
        .level(LogLevel::Error)
        .message("Failed to persist assurance report.")
        .error(expected_error.clone())
        .context("report_id=01TEST")
        .try_build();

    let request = is_ok!(result);
    assert_eq!(request.level(), LogLevel::Error);
    assert_eq!(request.message(), "Failed to persist assurance report.");
    assert_eq!(request.error(), Some(&expected_error));
    assert_eq!(request.context(), Some("report_id=01TEST"));
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that all log levels are valid request data for the single write-log
/// capability.
#[test]
fn request_builder_accepts_all_log_levels_success() {
    for level in [
        LogLevel::Error,
        LogLevel::Warning,
        LogLevel::Info,
        LogLevel::Debug,
    ] {
        let result = WriteLogEntryRequest::builder()
            .level(level)
            .message("Log entry.")
            .try_build();

        let request = is_ok!(result);
        assert_eq!(request.level(), level);
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the synchronous marker seam
/// writes one log entry.
#[test]
fn execute_through_marker_seam_writes_log_entry_success() {
    let gateway = WriteLogEntryFnGateway::new(write_log_entry_fixture);
    let request = request_fixture(LogLevel::Info);

    let result = Gateway::execute(&gateway as &dyn WriteLogEntryGW, request);

    is_ok!(result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the gateway through the asynchronous marker seam
/// writes one log entry.
#[test]
fn async_execute_through_marker_seam_writes_log_entry_success() {
    let gateway = WriteLogEntryFnGateway::new(write_log_entry_fixture);
    let request = request_fixture(LogLevel::Debug);

    let result = try_run_ready(AsyncGateway::execute(
        &gateway as &dyn AsyncWriteLogEntryGW,
        request,
    ));

    is_ok!(result);
}

fn request_fixture(level: LogLevel) -> WriteLogEntryRequest {
    WriteLogEntryRequest::builder()
        .level(level)
        .message("Application event.")
        .try_build()
        .expect("Expected write log entry request fixture to build.")
}

fn write_log_entry_fixture(request: WriteLogEntryRequest) -> Result<(), Error> {
    if request.message() == "Application event." {
        return Ok(());
    }

    Err(Error::for_system(
        Kind::ProcessingFailure,
        "The log entry fixture rejected the request.",
    ))
}

fn try_run_ready<Response>(mut future: ResponseFuture<'_, Response>) -> Result<Response, Error> {
    let mut context = Context::from_waker(std::task::Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("Expected gateway response future to be ready for verification."),
    }
}
