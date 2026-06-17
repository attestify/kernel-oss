//! Verifies the legacy identity gateway compatibility surface.
//!
//! Bounded unit under test:
//! - `IdentityGateway`
//! - `IdentityGatewayClone`
//!
//! Public interfaces verified:
//! - `IdentityGateway::generate`
//! - boxed trait-object cloning
//!
//! Logical paths covered:
//! - cloneable implementations can be cloned as boxed trait objects
//! - the legacy gateway contract returns the configured identity
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

#![allow(deprecated)]

use crate::error::Error;
use crate::gateway::identity::IdentityGateway;
use crate::ulid::ULID;
use test_framework_oss::is_ok;

#[derive(Clone)]
struct StaticIdentityGateway {
    identity: ULID,
}

impl IdentityGateway for StaticIdentityGateway {
    fn generate(&self) -> Result<ULID, Error> {
        Ok(self.identity)
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the legacy gateway returns the configured identity.
#[test]
fn generate_success() {
    let expected = ULID::nil();
    let gateway = StaticIdentityGateway { identity: expected };

    let actual = is_ok!(gateway.generate());

    assert_eq!(actual, expected);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that boxed legacy gateways can be cloned.
#[test]
fn clone_box_success() {
    let expected = ULID::nil();
    let gateway: Box<dyn IdentityGateway> = Box::new(StaticIdentityGateway { identity: expected });

    let cloned = gateway.clone();

    let actual = is_ok!(cloned.generate());
    assert_eq!(actual, expected);
}
