//! Verifies the new identity gateway seam.
//!
//! Bounded unit under test:
//! - `NewIdentityGW`
//!
//! Public interfaces verified:
//! - `VoidGateway::execute(&gateway as &dyn NewIdentityGW)`
//!
//! Logical paths covered:
//! - successful execution returns a bounded `ULID`
//! - boxed marker-seam execution returns a bounded `ULID`
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::error::Error;
use crate::gateway::VoidGateway;
use crate::gateway::new_identity::NewIdentityGW;
use crate::ulid::ULID;
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
