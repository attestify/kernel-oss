//! Verifies the current UTC timestamp gateway seam.
//!
//! Bounded unit under test:
//! - `CurrentUTCTimestampGW`
//!
//! Public interfaces verified:
//! - `VoidGateway::execute(&gateway as &dyn CurrentUTCTimestampGW)`
//!
//! Logical paths covered:
//! - successful execution returns the current UTC timestamp
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::Error;
use crate::gateway::VoidGateway;
use crate::gateway::current_utc_timestamp::CurrentUTCTimestampGW;
use crate::values::datetime::utc_timestamp::UTCTimestamp;
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

fn current_utc_timestamp_fixture() -> UTCTimestamp {
    UTCTimestamp::builder()
        .use_ns(1_716_167_033_123_456_789u128)
        .build()
        .expect("Expected UTC timestamp fixture to build for gateway verification.")
}
