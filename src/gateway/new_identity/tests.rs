//! Verifies the new identity gateway seam.
//!
//! Bounded unit under test:
//! - `NewIdentityGW`
//! - `AsyncNewIdentityGW`
//!
//! Public interfaces verified:
//! - `VoidGateway::execute(&gateway as &dyn NewIdentityGW)`
//! - `AsyncVoidGateway::execute(&gateway as &dyn AsyncNewIdentityGW)`
//!
//! Logical paths covered:
//! - successful execution returns a bounded `ULID`
//! - boxed marker-seam execution returns a bounded `ULID`
//! - asynchronous marker-seam execution returns a bounded `ULID`
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::core::traits::ResponseFuture;
use crate::error::Error;
use crate::gateway::new_identity::{AsyncNewIdentityGW, NewIdentityGW};
use crate::gateway::{AsyncVoidGateway, VoidGateway};
use crate::ulid::ULID;
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use test_framework_oss::is_ok;

struct StaticNewIdentityGateway {
    identity: ULID,
}

impl VoidGateway for StaticNewIdentityGateway {
    type Response = ULID;

    fn execute(&self) -> Result<Self::Response, Error> {
        Ok(self.identity)
    }
}

impl NewIdentityGW for StaticNewIdentityGateway {}

struct AsyncStaticNewIdentityGateway {
    identity: ULID,
}

impl AsyncVoidGateway for AsyncStaticNewIdentityGateway {
    type Response = ULID;

    fn execute<'a>(&'a self) -> ResponseFuture<'a, Self::Response> {
        let identity = self.identity;

        Box::pin(async move { Ok(identity) })
    }
}

impl AsyncNewIdentityGW for AsyncStaticNewIdentityGateway {}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the new identity gateway through the `NewIdentityGW`
/// marker seam returns the configured bounded `ULID` on the success path.
#[test]
fn execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway = StaticNewIdentityGateway { identity: expected };

    let result = VoidGateway::execute(&gateway as &dyn NewIdentityGW);

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing a boxed new identity gateway through the
/// `NewIdentityGW` marker seam returns the configured bounded `ULID` on the
/// success path.
#[test]
fn trait_object_execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway: Box<dyn NewIdentityGW> = Box::new(StaticNewIdentityGateway { identity: expected });

    let result = VoidGateway::execute(gateway.as_ref() as &dyn NewIdentityGW);

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the asynchronous new identity gateway through the
/// `AsyncNewIdentityGW` marker seam returns the configured bounded `ULID` on
/// the success path.
#[test]
fn async_execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway = AsyncStaticNewIdentityGateway { identity: expected };

    let result = try_run_ready(AsyncVoidGateway::execute(
        &gateway as &dyn AsyncNewIdentityGW,
    ));

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
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
        Poll::Pending => panic!("Expected gateway response future to be ready for verification."),
    }
}
