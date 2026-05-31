use crate::core::traits::ResponseFuture;
use crate::error::Error;
use crate::usecase::{AsyncUseCase, UseCase};

struct VoidUseCase;

impl UseCase for VoidUseCase {
	type Request = ();
	type Response = ();

	fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
		Ok(())
	}
}

struct VoidAsyncUseCase;

impl AsyncUseCase for VoidAsyncUseCase {
	type Request = ();
	type Response = ();

	fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
		Box::pin(async { Ok(()) })
	}
}

#[test]
fn sync_use_case_allows_unit_response_success() {
	let use_case = VoidUseCase;

	assert!(use_case.execute(()).is_ok());
}

#[test]
fn async_use_case_allows_unit_response_success() {
	let use_case = VoidAsyncUseCase;

	let _future: ResponseFuture<'_, ()> = use_case.execute(());
}