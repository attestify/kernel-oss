//! Verifies the new identity gateway seam.
//!
//! Bounded unit under test:
//! - `NewIdentityGW`
//! - `NewIdentityGatewayRequest`
//!
//! Public interfaces verified:
//! - `Gateway::execute(&gateway as &dyn NewIdentityGW, request)`
//!
//! Logical paths covered:
//! - successful execution returns a bounded `ULID`
//! - boxed marker-seam execution returns a bounded `ULID`
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::Error;
use crate::gateway::Gateway;
use crate::gateway::new_identity::{NewIdentityGW, NewIdentityGatewayRequest};
use crate::ulid::ULID;
use test_framework_oss::is_ok;

struct StaticNewIdentityGateway {
    identity: ULID,
}

impl Gateway for StaticNewIdentityGateway {
    type Request = NewIdentityGatewayRequest;
    type Response = ULID;

    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
        Ok(self.identity)
    }
}

impl NewIdentityGW for StaticNewIdentityGateway {}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that executing the new identity gateway through the `NewIdentityGW`
/// marker seam returns the configured bounded `ULID` on the success path.
#[test]
fn execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway = StaticNewIdentityGateway { identity: expected };
    let request = NewIdentityGatewayRequest;

    let result = Gateway::execute(&gateway as &dyn NewIdentityGW, request);

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
    let request = NewIdentityGatewayRequest;

    let result = Gateway::execute(gateway.as_ref() as &dyn NewIdentityGW, request);

    let actual = is_ok!(result);
    assert_eq!(expected, actual);
}
