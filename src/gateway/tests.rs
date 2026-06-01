use crate::core::traits::ResponseFuture;
use crate::error::Error;
use crate::gateway::{AsyncGateway, Gateway};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct VoidGatewayRequest;

struct VoidGateway;

impl Gateway for VoidGateway {
    type Request = VoidGatewayRequest;
    type Response = ();

    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
        Ok(())
    }
}

struct VoidAsyncGateway;

impl AsyncGateway for VoidAsyncGateway {
    type Request = VoidGatewayRequest;
    type Response = ();

    fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async { Ok(()) })
    }
}

#[test]
fn sync_gateway_allows_unit_response_success() {
    let gateway = VoidGateway;

    assert!(gateway.execute(VoidGatewayRequest).is_ok());
}

#[test]
fn async_gateway_allows_unit_response_success() {
    let gateway = VoidAsyncGateway;

    let _future: ResponseFuture<'_, ()> = gateway.execute(VoidGatewayRequest);
}
