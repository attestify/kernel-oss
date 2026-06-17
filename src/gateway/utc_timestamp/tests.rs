//! Verifies the legacy UTC timestamp gateway compatibility surface.
//!
//! Bounded unit under test:
//! - `UTCTimestampGateway`
//! - `UTCTimestampGatewayClone`
//!
//! Public interfaces verified:
//! - `UTCTimestampGateway::now`
//! - boxed trait-object cloning
//!
//! Logical paths covered:
//! - cloneable implementations can be cloned as boxed trait objects
//! - the legacy gateway contract returns the configured timestamp
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

#![allow(deprecated)]

use crate::error::Error;
use crate::gateway::utc_timestamp::UTCTimestampGateway;
use crate::values::datetime::utc_timestamp::UTCTimestamp;
use test_framework_oss::is_ok;

#[derive(Clone)]
struct StaticUTCTimestampGateway {
    timestamp: UTCTimestamp,
}

impl UTCTimestampGateway for StaticUTCTimestampGateway {
    fn now(&self) -> Result<UTCTimestamp, Error> {
        Ok(self.timestamp)
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the legacy gateway returns the configured timestamp.
#[test]
fn now_success() {
    let expected = is_ok!(UTCTimestamp::builder().use_ns(1_500_000u128).build());
    let gateway = StaticUTCTimestampGateway {
        timestamp: expected,
    };

    let actual = is_ok!(gateway.now());

    assert_eq!(actual, expected);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that boxed legacy gateways can be cloned.
#[test]
fn clone_box_success() {
    let expected = is_ok!(UTCTimestamp::builder().use_ns(1_500_000u128).build());
    let gateway: Box<dyn UTCTimestampGateway> = Box::new(StaticUTCTimestampGateway {
        timestamp: expected,
    });

    let cloned = gateway.clone();

    let actual = is_ok!(cloned.now());
    assert_eq!(actual, expected);
}
