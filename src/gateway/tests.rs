//! Verifies the shared gateway execution roles.
//!
//! Bounded unit under test:
//! - `VoidGateway`
//! - `Gateway`
//! - `AsyncVoidGateway`
//! - `AsyncGateway`
//! - `ResponseFuture`
//!
//! Public interfaces verified:
//! - `VoidGateway::execute()`
//! - `Gateway::execute(request)`
//! - `AsyncVoidGateway::execute()`
//! - `AsyncGateway::execute(request)`
//!
//! Logical paths covered:
//! - synchronous no-input gateway execution supports `Response = ()`
//! - synchronous gateway execution supports `Response = ()`
//! - asynchronous no-input gateway execution resolves a `ResponseFuture` for `Response = ()`
//! - asynchronous gateway execution resolves a `ResponseFuture` for `Response = ()`
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::Error;
use crate::gateway::{AsyncGateway, AsyncVoidGateway, Gateway, VoidGateway};
use crate::response::ResponseFuture;
use std::task::{Context, Poll};
use test_framework_oss::is_ok;

struct NoInputGateway;

impl VoidGateway for NoInputGateway {
    type Response = ();

    fn execute(&self) -> Result<Self::Response, Error> {
        Ok(())
    }
}

struct NoInputAsyncGateway;

impl AsyncVoidGateway for NoInputAsyncGateway {
    type Response = ();

    fn execute<'a>(&'a self) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async { Ok(()) })
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a synchronous gateway implementation can use `Response = ()`
/// while still returning a bounded `Result<(), Error>` through `execute`.
#[test]
fn sync_gateway_allows_unit_response_success() {
    let gateway = NoInputGateway;

    let result = VoidGateway::execute(&gateway);

    is_ok!(result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that an asynchronous gateway implementation can use `Response = ()`
/// while returning the shared `ResponseFuture` type from `execute` that resolves
/// to a bounded `Result<(), Error>`.
#[test]
fn async_gateway_allows_unit_response_success() {
    let gateway = NoInputAsyncGateway;

    let result = try_run_ready(AsyncVoidGateway::execute(&gateway));

    is_ok!(result);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ExampleRequest;

struct RequestGateway;

impl Gateway for RequestGateway {
    type Request = ExampleRequest;
    type Response = ();

    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
        Ok(())
    }
}

struct RequestAsyncGateway;

impl AsyncGateway for RequestAsyncGateway {
    type Request = ExampleRequest;
    type Response = ();

    fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async { Ok(()) })
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a synchronous request-bearing gateway implementation can use
/// `Response = ()` while still returning a bounded `Result<(), Error>` through
/// `execute`.
#[test]
fn sync_request_gateway_allows_unit_response_success() {
    let gateway = RequestGateway;

    let result = Gateway::execute(&gateway, ExampleRequest);

    is_ok!(result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that an asynchronous request-bearing gateway implementation can use
/// `Response = ()` while returning the shared `ResponseFuture` type from
/// `execute` that resolves to a bounded `Result<(), Error>`.
#[test]
fn async_request_gateway_allows_unit_response_success() {
    let gateway = RequestAsyncGateway;

    let result = try_run_ready(AsyncGateway::execute(&gateway, ExampleRequest));

    is_ok!(result);
}

fn try_run_ready<Response>(mut future: ResponseFuture<'_, Response>) -> Result<Response, Error> {
    let mut context = Context::from_waker(std::task::Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("Expected gateway response future to be ready for verification."),
    }
}
