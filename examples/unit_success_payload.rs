//! Demonstrates a no-payload success response carried through `Result`.
//!
//! This example shows a use case and gateway that use `type Response = ()`.
//! The successful payload is unit, but the seam still returns a real
//! `Result<(), Error>` so failures remain explicit and bounded.
//!
//! Run this example with:
//!
//! ```text
//! cargo run --example unit_success_payload
//! ```

use kernel_oss::error::{Error, Kind};
use kernel_oss::gateway::Gateway;
use kernel_oss::ulid::ULID;
use kernel_oss::usecase::UseCase;
use std::sync::Arc;

/// Represents the built request accepted by [`DeactivateUserUC`].
#[derive(Debug)]
struct DeactivateUserRequest {
    user_identity: ULID,
}

impl DeactivateUserRequest {
    /// Starts construction for a [`DeactivateUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a default [`DeactivateUserRequestBuilder`] for collecting and
    /// validating request input.
    fn builder() -> DeactivateUserRequestBuilder {
        DeactivateUserRequestBuilder::default()
    }

    /// Returns the validated user identity carried by the request.
    fn user_identity(&self) -> &ULID {
        &self.user_identity
    }
}

/// Builds a [`DeactivateUserRequest`] from raw caller input.
#[derive(Debug, Default)]
struct DeactivateUserRequestBuilder {
    user_identity: Option<u128>,
}

impl DeactivateUserRequestBuilder {
    /// Sets the raw user identity value for the request.
    ///
    /// # Arguments
    ///
    /// * `user_identity` - The raw user identity value required by the request.
    ///
    /// # Returns
    ///
    /// Returns the builder with the raw user identity value set.
    fn user_identity(mut self, user_identity: impl Into<u128>) -> Self {
        self.user_identity = Some(user_identity.into());
        self
    }

    /// Validates the builder state and constructs a [`DeactivateUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a built [`DeactivateUserRequest`] containing the validated user
    /// identity.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the user identity was not provided
    /// - the user identity is the nil [`ULID`]
    fn try_build(mut self) -> Result<DeactivateUserRequest, Error> {
        let user_identity = self.validate_user_identity()?;

        Ok(DeactivateUserRequest { user_identity })
    }

    fn validate_user_identity(&mut self) -> Result<ULID, Error> {
        let user_identity = self.user_identity.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A deactivate user identity is required, but none was provided.",
            )
        })?;
        let user_identity = ULID::from(user_identity);

        if user_identity.is_nil() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The deactivate user identity provided is nil, provide a non-nil deactivate user identity.",
            ));
        }

        Ok(user_identity)
    }
}

/// Represents the built request accepted by [`DeactivateUserGW`].
#[derive(Debug)]
struct DeactivateUserGatewayRequest {
    user_identity: ULID,
}

impl DeactivateUserGatewayRequest {
    /// Starts construction for a [`DeactivateUserGatewayRequest`].
    ///
    /// # Returns
    ///
    /// Returns a default [`DeactivateUserGatewayRequestBuilder`] for collecting
    /// and validating gateway input.
    fn builder() -> DeactivateUserGatewayRequestBuilder {
        DeactivateUserGatewayRequestBuilder::default()
    }

    /// Returns the validated user identity carried by the gateway request.
    fn user_identity(&self) -> &ULID {
        &self.user_identity
    }
}

/// Builds a [`DeactivateUserGatewayRequest`] from valid domain input.
#[derive(Debug, Default)]
struct DeactivateUserGatewayRequestBuilder {
    user_identity: Option<ULID>,
}

impl DeactivateUserGatewayRequestBuilder {
    /// Sets the validated user identity for the gateway request.
    ///
    /// # Arguments
    ///
    /// * `user_identity` - The validated user identity required by the gateway.
    ///
    /// # Returns
    ///
    /// Returns the builder with the user identity set.
    fn user_identity(mut self, user_identity: ULID) -> Self {
        self.user_identity = Some(user_identity);
        self
    }

    /// Validates the builder state and constructs a gateway request.
    ///
    /// # Returns
    ///
    /// Returns a built [`DeactivateUserGatewayRequest`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the user identity was not provided
    /// - the user identity is the nil [`ULID`]
    fn try_build(mut self) -> Result<DeactivateUserGatewayRequest, Error> {
        let user_identity = self.validate_user_identity()?;

        Ok(DeactivateUserGatewayRequest { user_identity })
    }

    fn validate_user_identity(&mut self) -> Result<ULID, Error> {
        let user_identity = self.user_identity.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A deactivate user gateway identity is required, but none was provided.",
            )
        })?;

        if user_identity.is_nil() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The deactivate user gateway identity provided is nil, provide a non-nil deactivate user gateway identity.",
            ));
        }

        Ok(user_identity)
    }
}

/// Domain-specific gateway seam for deactivating a user.
trait DeactivateUserGW: Gateway<Request = DeactivateUserGatewayRequest, Response = ()> {}

/// Domain-specific use case seam for deactivating a user.
trait DeactivateUserUC: UseCase<Request = DeactivateUserRequest, Response = ()> {}

/// Implements the deactivate-user use case.
struct DeactivateUser {
    deactivate_user_gateway: Arc<dyn DeactivateUserGW>,
}

impl DeactivateUser {
    /// Starts construction for a [`DeactivateUser`] use case.
    ///
    /// # Returns
    ///
    /// Returns a default [`DeactivateUserBuilder`] for collecting and validating
    /// required use case dependencies.
    fn builder() -> DeactivateUserBuilder {
        DeactivateUserBuilder::default()
    }
}

/// Builds a [`DeactivateUser`] use case implementation.
#[derive(Default)]
struct DeactivateUserBuilder {
    deactivate_user_gateway: Option<Arc<dyn DeactivateUserGW>>,
}

impl DeactivateUserBuilder {
    /// Sets the gateway seam used to deactivate users.
    ///
    /// # Arguments
    ///
    /// * `deactivate_user_gateway` - The gateway seam required by the use case.
    ///
    /// # Returns
    ///
    /// Returns the builder with the gateway seam set.
    fn deactivate_user_gateway(
        mut self,
        deactivate_user_gateway: Arc<dyn DeactivateUserGW>,
    ) -> Self {
        self.deactivate_user_gateway = Some(deactivate_user_gateway);
        self
    }

    /// Validates the builder state and constructs a [`DeactivateUser`].
    ///
    /// # Returns
    ///
    /// Returns a [`DeactivateUser`] containing the required gateway dependency.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the deactivate-user gateway dependency was not provided
    fn try_build(mut self) -> Result<DeactivateUser, Error> {
        let deactivate_user_gateway = self.validate_deactivate_user_gateway()?;

        Ok(DeactivateUser {
            deactivate_user_gateway,
        })
    }

    fn validate_deactivate_user_gateway(&mut self) -> Result<Arc<dyn DeactivateUserGW>, Error> {
        self.deactivate_user_gateway.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A deactivate user gateway is required, but none was provided.",
            )
        })
    }
}

impl UseCase for DeactivateUser {
    type Request = DeactivateUserRequest;
    type Response = ();

    /// Executes the use case and returns `Ok(())` when deactivation succeeds.
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let gateway_request = DeactivateUserGatewayRequest::builder()
            .user_identity(*request.user_identity())
            .try_build()?;

        Gateway::execute(
            self.deactivate_user_gateway.as_ref() as &dyn DeactivateUserGW,
            gateway_request,
        )?;

        Ok(())
    }
}

impl DeactivateUserUC for DeactivateUser {}

/// Represents the external capability used by the gateway implementation.
trait DeactivateUserClient: Send + Sync {
    /// Deactivates a user in the external capability.
    fn deactivate_user(&self, user_identity: &ULID) -> Result<(), Error>;
}

/// Provides a deterministic deactivate-user gateway for the example.
struct StaticDeactivateUserGateway {
    client: Arc<dyn DeactivateUserClient>,
}

impl StaticDeactivateUserGateway {
    /// Starts construction for a [`StaticDeactivateUserGateway`].
    ///
    /// # Returns
    ///
    /// Returns a default [`StaticDeactivateUserGatewayBuilder`] for collecting
    /// and validating gateway dependencies.
    fn builder() -> StaticDeactivateUserGatewayBuilder {
        StaticDeactivateUserGatewayBuilder::default()
    }
}

/// Builds a deterministic [`StaticDeactivateUserGateway`].
#[derive(Default)]
struct StaticDeactivateUserGatewayBuilder {
    client: Option<Arc<dyn DeactivateUserClient>>,
}

impl StaticDeactivateUserGatewayBuilder {
    /// Sets the external capability dependency for the gateway.
    ///
    /// # Arguments
    ///
    /// * `client` - The dependency used to deactivate users.
    ///
    /// # Returns
    ///
    /// Returns the builder with the dependency configured.
    fn client(mut self, client: Arc<dyn DeactivateUserClient>) -> Self {
        self.client = Some(client);
        self
    }

    /// Validates the builder state and constructs a static gateway.
    ///
    /// # Returns
    ///
    /// Returns a [`StaticDeactivateUserGateway`] configured with required
    /// dependencies.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the static gateway client was not provided
    fn try_build(mut self) -> Result<StaticDeactivateUserGateway, Error> {
        let client = self.validate_client()?;

        Ok(StaticDeactivateUserGateway { client })
    }

    fn validate_client(&mut self) -> Result<Arc<dyn DeactivateUserClient>, Error> {
        self.client.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A deactivate user client is required, but none was provided.",
            )
        })
    }
}

impl Gateway for StaticDeactivateUserGateway {
    type Request = DeactivateUserGatewayRequest;
    type Response = ();

    /// Executes the gateway and returns `Ok(())` when deactivation succeeds.
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        self.client.deactivate_user(request.user_identity())?;

        Ok(())
    }
}

impl DeactivateUserGW for StaticDeactivateUserGateway {}

/// Provides a dependency implementation that succeeds with no payload.
struct SuccessfulDeactivateUserClient;

impl DeactivateUserClient for SuccessfulDeactivateUserClient {
    fn deactivate_user(&self, _user_identity: &ULID) -> Result<(), Error> {
        Ok(())
    }
}

/// Provides a dependency implementation that fails with a bounded error.
struct FailingDeactivateUserClient;

impl DeactivateUserClient for FailingDeactivateUserClient {
    fn deactivate_user(&self, _user_identity: &ULID) -> Result<(), Error> {
        Err(deactivate_user_client_error())
    }
}

fn deactivate_user_client_error() -> Error {
    Error::for_system(Kind::GatewayError, "The deactivate user client failed.")
}

/// Runs the unit-success-payload example.
///
/// # Errors
///
/// Returns an error if:
/// - any request or use case builder is missing required input
/// - any request user identity is invalid
fn main() -> Result<(), Error> {
    let user_identity = ULID::from_parts(1, 21);
    let request = DeactivateUserRequest::builder()
        .user_identity(user_identity.value())
        .try_build()?;
    let deactivate_user_gateway: Arc<dyn DeactivateUserGW> = Arc::new(
        StaticDeactivateUserGateway::builder()
            .client(Arc::new(SuccessfulDeactivateUserClient))
            .try_build()?,
    );
    let use_case = DeactivateUser::builder()
        .deactivate_user_gateway(deactivate_user_gateway)
        .try_build()?;

    let success = UseCase::execute(&use_case as &dyn DeactivateUserUC, request);

    assert_eq!(success, Ok(()));

    let failure_error = deactivate_user_client_error();
    let request = DeactivateUserRequest::builder()
        .user_identity(user_identity.value())
        .try_build()?;
    let deactivate_user_gateway: Arc<dyn DeactivateUserGW> = Arc::new(
        StaticDeactivateUserGateway::builder()
            .client(Arc::new(FailingDeactivateUserClient))
            .try_build()?,
    );
    let use_case = DeactivateUser::builder()
        .deactivate_user_gateway(deactivate_user_gateway)
        .try_build()?;

    let failure = UseCase::execute(&use_case as &dyn DeactivateUserUC, request);

    assert_eq!(failure, Err(failure_error));

    Ok(())
}
