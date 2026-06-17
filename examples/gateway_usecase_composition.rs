//! Demonstrates standards-aligned gateway and use case composition.
//!
//! This example shows how a caller builds a request before crossing the use
//! case boundary, how a use case calls a gateway through a domain-specific
//! marker seam, and how a single bounded entity can be returned directly as the
//! successful response.
//!
//! Run this example with:
//!
//! ```text
//! cargo run --example gateway_usecase_composition
//! ```

use kernel_oss::entity::Entity;
use kernel_oss::error::{Error, Kind};
use kernel_oss::gateway::VoidGateway;
use kernel_oss::gateway::new_identity::NewIdentityGW;
use kernel_oss::ulid::ULID;
use kernel_oss::usecase::UseCase;
use kernel_oss::values::Value;
use std::fmt;
use std::sync::Arc;

/// Represents a validated email address value object.
///
/// The canonical value is exposed as a string slice through [`Value`]. The
/// value object stores normalized lowercase text after construction succeeds.
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

/// Represents the registered user returned by the use case.
///
/// The registered user is the successful bounded response for the example use
/// case, so no separate response wrapper is needed. Its identity is a direct
/// [`ULID`] rather than a user-specific identity wrapper. Equality is
/// identity-based.
#[derive(Clone, Debug)]
struct RegisteredUser {
    id: ULID,
    email: EmailAddress,
}

impl RegisteredUser {
    /// Creates a [`RegisteredUser`] from a validated identity and email address.
    ///
    /// # Arguments
    ///
    /// * `id` - The stable identity for the registered user entity.
    /// * `email` - The validated email address for the registered user.
    fn new(id: ULID, email: EmailAddress) -> Self {
        Self { id, email }
    }

    /// Returns the registered user's email address.
    fn email(&self) -> &EmailAddress {
        &self.email
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

/// Represents the built request accepted by [`RegisterUserUC`].
///
/// This is a non-void request object, so callers start construction with
/// [`RegisterUserRequest::builder`] and finalize the request before calling
/// the use case.
#[derive(Debug)]
struct RegisterUserRequest {
    email_address: EmailAddress,
}

impl RegisterUserRequest {
    /// Starts construction for a [`RegisterUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a default [`RegisterUserRequestBuilder`] for collecting and
    /// validating request input.
    fn builder() -> RegisterUserRequestBuilder {
        RegisterUserRequestBuilder::default()
    }

    /// Returns the validated email address carried by the request.
    fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }
}

/// Builds a [`RegisterUserRequest`] for the register-user use case seam.
///
/// The builder accepts raw request input, validates it during
/// [`RegisterUserRequestBuilder::try_build`], and returns only a finalized
/// request to the use case.
#[derive(Debug, Default)]
struct RegisterUserRequestBuilder {
    email_address: Option<String>,
}

impl RegisterUserRequestBuilder {
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

    /// Validates the builder state and constructs a [`RegisterUserRequest`].
    ///
    /// # Returns
    ///
    /// Returns a built [`RegisterUserRequest`] containing the validated email
    /// address.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the email address was not provided
    /// - the raw email address is empty or contains only whitespace
    /// - the raw email address does not contain `@`
    fn try_build(mut self) -> Result<RegisterUserRequest, Error> {
        let email_address = self.validate_email_address()?;

        Ok(RegisterUserRequest { email_address })
    }

    fn validate_email_address(&mut self) -> Result<EmailAddress, Error> {
        let email_address = self.email_address.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A register user email address is required, but none was provided.",
            )
        })?;

        if email_address.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The register user email address provided is empty, provide a non-empty register user email address.",
            ));
        }

        if email_address.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The register user email address provided contains only whitespace, provide a non-empty register user email address.",
            ));
        }

        EmailAddress::try_new(email_address)
    }
}

/// Defines the synchronous register-user use case seam.
///
/// This marker trait binds the shared [`UseCase`] trait to a built
/// [`RegisterUserRequest`] and a bounded [`RegisteredUser`] response. The
/// shared use case trait supplies the `Send + Sync` bound and the
/// `execute(request)` contract.
trait RegisterUserUC: UseCase<Request = RegisterUserRequest, Response = RegisteredUser> {}

/// Implements the register-user use case.
///
/// The use case depends on [`NewIdentityGW`] to obtain a new identity and then
/// composes the successful [`RegisteredUser`] response. Construct it through
/// [`RegisterUser::builder`] so required dependencies are verified before use.
struct RegisterUser {
    new_identity_gateway: Arc<dyn NewIdentityGW>,
}

impl RegisterUser {
    /// Starts construction for a [`RegisterUser`] use case.
    ///
    /// # Returns
    ///
    /// Returns a default [`RegisterUserBuilder`] for collecting and validating
    /// required use case dependencies.
    fn builder() -> RegisterUserBuilder {
        RegisterUserBuilder::default()
    }
}

/// Builds a [`RegisterUser`] use case implementation.
///
/// The builder verifies that the required new-identity gateway dependency is
/// provided before the use case is constructed.
#[derive(Default)]
struct RegisterUserBuilder {
    new_identity_gateway: Option<Arc<dyn NewIdentityGW>>,
}

impl RegisterUserBuilder {
    /// Sets the gateway seam used to request new user identities.
    ///
    /// # Arguments
    ///
    /// * `new_identity_gateway` - The gateway seam required by the use case.
    ///
    /// # Returns
    ///
    /// Returns the builder with the new-identity gateway set.
    fn new_identity_gateway(mut self, new_identity_gateway: Arc<dyn NewIdentityGW>) -> Self {
        self.new_identity_gateway = Some(new_identity_gateway);
        self
    }

    /// Validates the builder state and constructs a [`RegisterUser`].
    ///
    /// # Returns
    ///
    /// Returns a [`RegisterUser`] containing the required gateway dependency.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the new-identity gateway dependency was not provided
    fn try_build(mut self) -> Result<RegisterUser, Error> {
        let new_identity_gateway = self.validate_new_identity_gateway()?;

        Ok(RegisterUser {
            new_identity_gateway,
        })
    }

    fn validate_new_identity_gateway(&mut self) -> Result<Arc<dyn NewIdentityGW>, Error> {
        self.new_identity_gateway.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A new identity gateway is required, but none was provided.",
            )
        })
    }
}

impl UseCase for RegisterUser {
    type Request = RegisterUserRequest;
    type Response = RegisteredUser;

    /// Executes the register-user use case with a built request object.
    ///
    /// # Arguments
    ///
    /// * `request` - The built request containing the validated email address.
    ///
    /// # Returns
    ///
    /// Returns the registered user entity created by the use case.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the new identity gateway fails to return an identity
    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let email_address = request.email_address().clone();
        let identity =
            VoidGateway::execute(self.new_identity_gateway.as_ref() as &dyn NewIdentityGW)?;

        Ok(RegisteredUser::new(identity, email_address))
    }
}

impl RegisterUserUC for RegisterUser {}

/// Provides a deterministic new-identity gateway for the example.
///
/// This gateway implements [`NewIdentityGW`] by returning a configured
/// [`ULID`], which keeps the example deterministic. Construct it through
/// [`StaticNewIdentityGateway::builder`] so required configuration is verified.
struct StaticNewIdentityGateway {
    identity: ULID,
}

impl StaticNewIdentityGateway {
    /// Starts construction for a [`StaticNewIdentityGateway`].
    ///
    /// # Returns
    ///
    /// Returns a default [`StaticNewIdentityGatewayBuilder`] for collecting and
    /// validating gateway configuration.
    fn builder() -> StaticNewIdentityGatewayBuilder {
        StaticNewIdentityGatewayBuilder::default()
    }
}

/// Builds a deterministic [`StaticNewIdentityGateway`].
///
/// The builder verifies that the identity returned by the gateway is provided.
#[derive(Default)]
struct StaticNewIdentityGatewayBuilder {
    identity: Option<ULID>,
}

impl StaticNewIdentityGatewayBuilder {
    /// Sets the identity returned by the gateway.
    ///
    /// # Arguments
    ///
    /// * `identity` - The [`ULID`] returned by the gateway.
    ///
    /// # Returns
    ///
    /// Returns the builder with the identity set.
    fn identity(mut self, identity: ULID) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Validates the builder state and constructs a [`StaticNewIdentityGateway`].
    ///
    /// # Returns
    ///
    /// Returns a [`StaticNewIdentityGateway`] configured with the provided
    /// identity.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the static identity was not provided
    fn try_build(mut self) -> Result<StaticNewIdentityGateway, Error> {
        let identity = self.validate_identity()?;

        Ok(StaticNewIdentityGateway { identity })
    }

    fn validate_identity(&mut self) -> Result<ULID, Error> {
        self.identity.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A static identity is required, but none was provided.",
            )
        })
    }
}

impl VoidGateway for StaticNewIdentityGateway {
    type Response = ULID;

    /// Executes the new-identity gateway without a request object.
    ///
    /// # Returns
    ///
    /// Returns the configured [`ULID`] directly as the bounded response.
    ///
    /// # Errors
    ///
    /// This deterministic example implementation does not return an error.
    fn execute(&self) -> Result<Self::Response, Error> {
        Ok(self.identity)
    }
}

impl NewIdentityGW for StaticNewIdentityGateway {}

/// Runs the gateway and use case composition example.
///
/// # Errors
///
/// Returns an error if:
/// - the example email address is empty or contains only whitespace
/// - the example email address does not contain `@`
/// - the register-user request builder does not receive an email address
/// - the new identity gateway fails to return an identity
fn main() -> Result<(), Error> {
    let generated_identity = ULID::from_parts(1, 42);
    let new_identity_gateway: Arc<dyn NewIdentityGW> = Arc::new(
        StaticNewIdentityGateway::builder()
            .identity(generated_identity)
            .try_build()?,
    );
    let use_case = RegisterUser::builder()
        .new_identity_gateway(new_identity_gateway)
        .try_build()?;

    let request = RegisterUserRequest::builder()
        .email_address("OWNER@example.com")
        .try_build()?;

    let user = UseCase::execute(&use_case as &dyn RegisterUserUC, request)?;

    assert_eq!(user.email().value(), "owner@example.com");
    assert_eq!(user.id(), &generated_identity);
    assert_eq!(user.id().value(), generated_identity.value());

    Ok(())
}
