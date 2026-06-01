//! Verifies the shared use case execution roles.
//!
//! Bounded unit under test:
//! - `UseCase`
//! - `AsyncUseCase`
//! - `ResponseFuture`
//!
//! Public interfaces verified:
//! - `UseCase::execute(request)`
//! - `AsyncUseCase::execute(request)`
//!
//! Logical paths covered:
//! - synchronous use case execution supports `Response = ()`
//! - asynchronous use case execution resolves a `ResponseFuture` for `Response = ()`
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::core::traits::ResponseFuture;
use crate::error::Error;
use crate::usecase::{AsyncUseCase, UseCase};
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use test_framework_oss::is_ok;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct VoidUseCaseRequest;

struct VoidUseCase;

impl UseCase for VoidUseCase {
    type Request = VoidUseCaseRequest;
    type Response = ();

    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
        Ok(())
    }
}

struct VoidAsyncUseCase;

impl AsyncUseCase for VoidAsyncUseCase {
    type Request = VoidUseCaseRequest;
    type Response = ();

    fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async { Ok(()) })
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a synchronous use case implementation can use `Response = ()`
/// while still returning a bounded `Result<(), Error>` through `execute`.
#[test]
fn sync_use_case_allows_unit_response_success() {
    let use_case = VoidUseCase;

    let result = UseCase::execute(&use_case, VoidUseCaseRequest);

    is_ok!(result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that an asynchronous use case implementation can use `Response = ()`
/// while returning the shared `ResponseFuture` type from `execute` that resolves
/// to a bounded `Result<(), Error>`.
#[test]
fn async_use_case_allows_unit_response_success() {
    let use_case = VoidAsyncUseCase;

    let result = try_run_ready(AsyncUseCase::execute(&use_case, VoidUseCaseRequest));

    is_ok!(result);
}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn try_run_ready<Response>(mut future: ResponseFuture<'_, Response>) -> Result<Response, Error> {
    let waker = std::task::Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("Expected use case response future to be ready for verification."),
    }
}
