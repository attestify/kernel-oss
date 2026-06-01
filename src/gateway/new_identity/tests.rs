use crate::error::Error;
use crate::gateway::Gateway;
use crate::gateway::new_identity::{NewIdentityGW, NewIdentityGatewayRequest};
use crate::ulid::ULID;

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

#[test]
fn execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway = StaticNewIdentityGateway { identity: expected };
    let request = NewIdentityGatewayRequest;

    let actual = gateway
        .execute(request)
        .expect("Expected new identity gateway execution to succeed.");

    assert_eq!(expected, actual);
}

#[test]
fn trait_object_execute_returns_new_identity_success() {
    let expected = ULID::nil();
    let gateway: Box<dyn NewIdentityGW> = Box::new(StaticNewIdentityGateway { identity: expected });
    let request = NewIdentityGatewayRequest;

    let actual = gateway
        .execute(request)
        .expect("Expected new identity gateway trait object execution to succeed.");

    assert_eq!(expected, actual);
}
