//! Verifies the current UTC timestamp gateway seam.
//!
//! Bounded unit under test:
//! - `CurrentUTCTimestampGW`
//! - `AsyncCurrentUTCTimestampGW`
//!
//! Public interfaces verified:
//! - `VoidGateway::execute(&gateway as &dyn CurrentUTCTimestampGW)`
//! - `AsyncVoidGateway::execute(&gateway as &dyn AsyncCurrentUTCTimestampGW)`
//!
//! Logical paths covered:
//! - successful execution returns the current UTC timestamp
//! - asynchronous execution returns the current UTC timestamp
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::Error;
use crate::gateway::current_utc_timestamp::{AsyncCurrentUTCTimestampGW, CurrentUTCTimestampGW};
use crate::gateway::{AsyncVoidGateway, VoidGateway};
use crate::response::ResponseFuture;
use crate::values::datetime::utc_timestamp::UTCTimestamp;
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use test_framework_oss::is_ok;

struct StaticCurrentUTCTimestampGateway {
    timestamp: UTCTimestamp,
}

impl VoidGateway for StaticCurrentUTCTimestampGateway {
    type Response = UTCTimestamp;

    fn execute(&self) -> Result<Self::Response, Error> {
        Ok(self.timestamp)
    }
}

impl CurrentUTCTimestampGW for StaticCurrentUTCTimestampGateway {}

struct AsyncStaticCurrentUTCTimestampGateway {
    timestamp: UTCTimestamp,
}

impl AsyncVoidGateway for AsyncStaticCurrentUTCTimestampGateway {
    type Response = UTCTimestamp;

    fn execute<'a>(&'a self) -> ResponseFuture<'a, Self::Response> {
        let timestamp = self.timestamp;

        Box::pin(async move { Ok(timestamp) })
    }
}

impl AsyncCurrentUTCTimestampGW for AsyncStaticCurrentUTCTimestampGateway {}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the current UTC timestamp gateway through the
/// `CurrentUTCTimestampGW` marker seam returns the configured bounded
/// `UTCTimestamp` on the success path.
#[test]
fn execute_through_marker_seam_returns_current_utc_timestamp_success() {
    let expected = current_utc_timestamp_fixture();
    let gateway = StaticCurrentUTCTimestampGateway {
        timestamp: expected,
    };

    let result = VoidGateway::execute(&gateway as &dyn CurrentUTCTimestampGW);

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the asynchronous current UTC timestamp gateway
/// through the `AsyncCurrentUTCTimestampGW` marker seam returns the configured
/// bounded `UTCTimestamp` on the success path.
#[test]
fn async_execute_through_marker_seam_returns_current_utc_timestamp_success() {
    let expected = current_utc_timestamp_fixture();
    let gateway = AsyncStaticCurrentUTCTimestampGateway {
        timestamp: expected,
    };

    let result = try_run_ready(AsyncVoidGateway::execute(
        &gateway as &dyn AsyncCurrentUTCTimestampGW,
    ));

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
}

fn current_utc_timestamp_fixture() -> UTCTimestamp {
    UTCTimestamp::builder()
        .use_ns(1_716_167_033_123_456_789u128)
        .build()
        .expect("Expected UTC timestamp fixture to build for gateway verification.")
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
