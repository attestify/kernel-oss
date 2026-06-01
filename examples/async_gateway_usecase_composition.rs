//! Demonstrates async gateway and use case composition.
//!
//! This example shows async seam traits using explicit [`ResponseFuture`]
//! return values from normal `execute` methods. The use case composes an async
//! gateway without exposing a public async function.
//!
//! Run this example with:
//!
//! ```text
//! cargo run --example async_gateway_usecase_composition
//! ```

use kernel_oss::core::traits::{Entity, ResponseFuture};
use kernel_oss::error::{Error, Kind};
use kernel_oss::gateway::AsyncGateway;
use kernel_oss::ulid::ULID;
use kernel_oss::usecase::AsyncUseCase;
use kernel_oss::values::Value;
use std::fmt;
use std::sync::Arc;

/// Represents a validated email address value object.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EmailAddress {
    value: String,
}

impl EmailAddress {
    /// Validates and constructs an [`EmailAddress`].
    ///
    /// # Arguments
    ///
    /// * `value` - The raw email address text to trim, lowercase, and validate.
    ///
    /// # Returns
    ///
    /// Returns a new [`EmailAddress`] containing the normalized email address.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the email address is empty or contains only whitespace
    /// - the email address does not contain `@`
    fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into().trim().to_ascii_lowercase();

        if value.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "email address is required",
            ));
        }

        if !value.contains('@') {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "email address must contain @",
            ));
        }

        Ok(Self { value })
    }
}

impl Value for EmailAddress {
    type ValueType = str;

    /// Returns the normalized email address text.
    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

impl fmt::Display for EmailAddress {
    /// Formats the normalized email address text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

/// Represents the registered user returned by the async use case.
#[derive(Clone, Debug)]
struct RegisteredUser {
    id: ULID,
    email_address: EmailAddress,
}

impl RegisteredUser {
    /// Creates a [`RegisteredUser`] from a validated identity and email address.
    ///
    /// # Arguments
    ///
    /// * `id` - The stable identity for the registered user entity.
    /// * `email_address` - The validated email address for the registered user.
    fn new(id: ULID, email_address: EmailAddress) -> Self {
        Self { id, email_address }
    }

    /// Returns the registered user's email address.
    fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

impl Entity for RegisteredUser {
    type IdType = ULID;

    /// Returns the stable registered user identity.
    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

impl PartialEq for RegisteredUser {
    /// Compares registered users by stable identity.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RegisteredUser {}

/// Represents the built request accepted by [`LoadRegisteredUserUC`].
#[derive(Debug)]
struct LoadRegisteredUserRequest {
    email_address: EmailAddress,
}

impl LoadRegisteredUserRequest {
    /// Starts construction for a [`LoadRegisteredUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a default [`LoadRegisteredUserRequestBuilder`] for collecting
    /// and validating request input.
    fn builder() -> LoadRegisteredUserRequestBuilder {
        LoadRegisteredUserRequestBuilder::default()
    }

    /// Returns the validated email address carried by the request.
    fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

/// Builds a [`LoadRegisteredUserRequest`] from raw caller input.
#[derive(Debug, Default)]
struct LoadRegisteredUserRequestBuilder {
    email_address: Option<String>,
}

impl LoadRegisteredUserRequestBuilder {
    /// Sets the raw email address text for the request.
    ///
    /// # Arguments
    ///
    /// * `email_address` - The raw email address text required by the request.
    ///
    /// # Returns
    ///
    /// Returns the builder with the raw email address text set.
    fn email_address(mut self, email_address: impl Into<String>) -> Self {
        self.email_address = Some(email_address.into());
        self
    }

    /// Validates the builder state and constructs a [`LoadRegisteredUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a built [`LoadRegisteredUserRequest`] containing the validated
    /// email address.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the email address was not provided
    /// - the raw email address is empty or contains only whitespace
    /// - the raw email address does not contain `@`
    fn try_build(mut self) -> Result<LoadRegisteredUserRequest, Error> {
        let email_address = self.validate_email_address()?;

        Ok(LoadRegisteredUserRequest { email_address })
    }

    fn validate_email_address(&mut self) -> Result<EmailAddress, Error> {
        let email_address = self.email_address.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A load registered user email address is required, but none was provided.",
            )
        })?;

        if email_address.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The load registered user email address provided is empty, provide a non-empty load registered user email address.",
            ));
        }

        if email_address.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The load registered user email address provided contains only whitespace, provide a non-empty load registered user email address.",
            ));
        }

        EmailAddress::try_new(email_address)
    }
}

/// Represents the built request accepted by [`FindRegisteredUserGW`].
#[derive(Debug)]
struct FindRegisteredUserGatewayRequest {
    email_address: EmailAddress,
}

impl FindRegisteredUserGatewayRequest {
    /// Starts construction for a [`FindRegisteredUserGatewayRequest`].
    ///
    /// # Returns
    ///
    /// Returns a default [`FindRegisteredUserGatewayRequestBuilder`] for
    /// collecting and validating gateway input.
    fn builder() -> FindRegisteredUserGatewayRequestBuilder {
        FindRegisteredUserGatewayRequestBuilder::default()
    }

    /// Returns the validated email address carried by the gateway request.
    fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

/// Builds a [`FindRegisteredUserGatewayRequest`] from valid domain input.
#[derive(Debug, Default)]
struct FindRegisteredUserGatewayRequestBuilder {
    email_address: Option<EmailAddress>,
}

impl FindRegisteredUserGatewayRequestBuilder {
    /// Sets the validated email address for the gateway request.
    ///
    /// # Arguments
    ///
    /// * `email_address` - The validated email address required by the gateway.
    ///
    /// # Returns
    ///
    /// Returns the builder with the email address set.
    fn email_address(mut self, email_address: EmailAddress) -> Self {
        self.email_address = Some(email_address);
        self
    }

    /// Validates the builder state and constructs a gateway request.
    ///
    /// # Returns
    ///
    /// Returns a built [`FindRegisteredUserGatewayRequest`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the email address was not provided
    fn try_build(mut self) -> Result<FindRegisteredUserGatewayRequest, Error> {
        let email_address = self.validate_email_address()?;

        Ok(FindRegisteredUserGatewayRequest { email_address })
    }

    fn validate_email_address(&mut self) -> Result<EmailAddress, Error> {
        self.email_address.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A find registered user email address is required, but none was provided.",
            )
        })
    }
}

/// Domain-specific async gateway seam for finding a registered user.
trait FindRegisteredUserGW:
    AsyncGateway<Request = FindRegisteredUserGatewayRequest, Response = RegisteredUser>
{
}

/// Domain-specific async use case seam for loading a registered user.
trait LoadRegisteredUserUC:
    AsyncUseCase<Request = LoadRegisteredUserRequest, Response = RegisteredUser>
{
}

/// Implements the async load-registered-user use case.
struct LoadRegisteredUser {
    find_registered_user_gateway: Arc<dyn FindRegisteredUserGW>,
}

impl LoadRegisteredUser {
    /// Starts construction for a [`LoadRegisteredUser`] use case.
    ///
    /// # Returns
    ///
    /// Returns a default [`LoadRegisteredUserBuilder`] for collecting and
    /// validating required use case dependencies.
    fn builder() -> LoadRegisteredUserBuilder {
        LoadRegisteredUserBuilder::default()
    }
}

/// Builds a [`LoadRegisteredUser`] use case implementation.
#[derive(Default)]
struct LoadRegisteredUserBuilder {
    find_registered_user_gateway: Option<Arc<dyn FindRegisteredUserGW>>,
}

impl LoadRegisteredUserBuilder {
    /// Sets the async gateway seam used to find registered users.
    ///
    /// # Arguments
    ///
    /// * `find_registered_user_gateway` - The gateway seam required by the use case.
    ///
    /// # Returns
    ///
    /// Returns the builder with the async gateway seam set.
    fn find_registered_user_gateway(
        mut self,
        find_registered_user_gateway: Arc<dyn FindRegisteredUserGW>,
    ) -> Self {
        self.find_registered_user_gateway = Some(find_registered_user_gateway);
        self
    }

    /// Validates the builder state and constructs a [`LoadRegisteredUser`].
    ///
    /// # Returns
    ///
    /// Returns a [`LoadRegisteredUser`] containing the required gateway dependency.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the find-registered-user gateway dependency was not provided
    fn try_build(mut self) -> Result<LoadRegisteredUser, Error> {
        let find_registered_user_gateway = self.validate_find_registered_user_gateway()?;

        Ok(LoadRegisteredUser {
            find_registered_user_gateway,
        })
    }

    fn validate_find_registered_user_gateway(
        &mut self,
    ) -> Result<Arc<dyn FindRegisteredUserGW>, Error> {
        self.find_registered_user_gateway.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A find registered user gateway is required, but none was provided.",
            )
        })
    }
}

impl AsyncUseCase for LoadRegisteredUser {
    type Request = LoadRegisteredUserRequest;
    type Response = RegisteredUser;

    /// Executes the use case and returns an explicit response future.
    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let gateway_request = FindRegisteredUserGatewayRequest::builder()
                .email_address(request.email_address().clone())
                .try_build()?;

            AsyncGateway::execute(
                self.find_registered_user_gateway.as_ref() as &dyn FindRegisteredUserGW,
                gateway_request,
            )
            .await
        })
    }
}

impl LoadRegisteredUserUC for LoadRegisteredUser {}

/// Provides a deterministic async gateway for the example.
struct StaticFindRegisteredUserGateway {
    identity: ULID,
}

impl StaticFindRegisteredUserGateway {
    /// Starts construction for a [`StaticFindRegisteredUserGateway`].
    ///
    /// # Returns
    ///
    /// Returns a default [`StaticFindRegisteredUserGatewayBuilder`] for
    /// collecting and validating gateway configuration.
    fn builder() -> StaticFindRegisteredUserGatewayBuilder {
        StaticFindRegisteredUserGatewayBuilder::default()
    }
}

/// Builds a deterministic [`StaticFindRegisteredUserGateway`].
#[derive(Default)]
struct StaticFindRegisteredUserGatewayBuilder {
    identity: Option<ULID>,
}

impl StaticFindRegisteredUserGatewayBuilder {
    /// Sets the identity returned on the registered user.
    ///
    /// # Arguments
    ///
    /// * `identity` - The [`ULID`] used as the returned user identity.
    ///
    /// # Returns
    ///
    /// Returns the builder with the identity set.
    fn identity(mut self, identity: ULID) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Validates the builder state and constructs a static async gateway.
    ///
    /// # Returns
    ///
    /// Returns a [`StaticFindRegisteredUserGateway`] configured with an identity.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the static identity was not provided
    fn try_build(mut self) -> Result<StaticFindRegisteredUserGateway, Error> {
        let identity = self.validate_identity()?;

        Ok(StaticFindRegisteredUserGateway { identity })
    }

    fn validate_identity(&mut self) -> Result<ULID, Error> {
        self.identity.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A static registered user identity is required, but none was provided.",
            )
        })
    }
}

impl AsyncGateway for StaticFindRegisteredUserGateway {
    type Request = FindRegisteredUserGatewayRequest;
    type Response = RegisteredUser;

    /// Executes the gateway and returns an explicit response future.
    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            Ok(RegisteredUser::new(
                self.identity,
                request.email_address().clone(),
            ))
        })
    }
}

impl FindRegisteredUserGW for StaticFindRegisteredUserGateway {}

mod example_runtime {
    //! Local runner for this standalone example.
    //!
    //! The async seam standard defines the shape of the returned future:
    //! [`ResponseFuture`]. It does not define how application futures are
    //! scheduled. This crate intentionally has no async runtime dependency, so
    //! this example includes the smallest possible runner for futures that are
    //! ready immediately.
    //!
    //! A running application should replace this module with its real executor
    //! or host runtime. For example, a service might use Tokio, async-std, smol,
    //! Actix, or a framework-provided task runtime. A command-line binary would
    //! usually call the runtime's `block_on` entrypoint, and request handlers
    //! would normally wait inside runtime-managed tasks.

    use super::{Error, Kind, ResponseFuture};
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    /// Provides a no-op wake implementation for polling ready example futures.
    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    /// Polls a future that is expected to be ready immediately in this example.
    ///
    /// # Arguments
    ///
    /// * `future` - The explicit response future returned by an async seam.
    ///
    /// # Returns
    ///
    /// Returns the completed response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the future returns an error
    /// - the future is pending and needs a real executor
    pub(super) fn run_ready<Response>(
        mut future: ResponseFuture<'_, Response>,
    ) -> Result<Response, Error> {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => result,
            Poll::Pending => Err(Error::for_system(
                Kind::ProcessingFailure,
                "The async example future is pending, provide an executor to complete it.",
            )),
        }
    }
}

/// Runs the async gateway and use case composition example.
///
/// # Errors
///
/// Returns an error if:
/// - the example email address is invalid
/// - the use case or gateway builders are missing required input
/// - the example future is pending and needs a real executor
fn main() -> Result<(), Error> {
    let generated_identity = ULID::from_parts(1, 84);
    let find_registered_user_gateway: Arc<dyn FindRegisteredUserGW> = Arc::new(
        StaticFindRegisteredUserGateway::builder()
            .identity(generated_identity)
            .try_build()?,
    );
    let use_case = LoadRegisteredUser::builder()
        .find_registered_user_gateway(find_registered_user_gateway)
        .try_build()?;

    let request = LoadRegisteredUserRequest::builder()
        .email_address("OWNER@example.com")
        .try_build()?;

    let user = example_runtime::run_ready(AsyncUseCase::execute(
        &use_case as &dyn LoadRegisteredUserUC,
        request,
    ))?;

    assert_eq!(user.email_address().value(), "owner@example.com");
    assert_eq!(user.id(), &generated_identity);
    assert_eq!(user.id().value(), generated_identity.value());

    Ok(())
}
