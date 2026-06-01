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

use kernel_oss::core::traits::Entity;
use kernel_oss::error::{Error, Kind};
use kernel_oss::gateway::Gateway;
use kernel_oss::gateway::new_identity::{NewIdentityGW, NewIdentityGatewayRequest};
use kernel_oss::ulid::ULID;
use kernel_oss::usecase::UseCase;
use kernel_oss::values::Value;
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

/// Represents a user identity value object.
///
/// The canonical value is exposed as a `u128` through [`Value`], matching the
/// primitive value carried by [`ULID`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct UserId {
    value: ULID,
}

impl UserId {
    /// Creates a [`UserId`] from an existing [`ULID`].
    ///
    /// # Arguments
    ///
    /// * `value` - The bounded [`ULID`] value that identifies the user.
    fn new(value: ULID) -> Self {
        Self { value }
    }
}

impl Value for UserId {
    type ValueType = u128;

    /// Returns the primitive `u128` identity value.
    fn value(&self) -> &Self::ValueType {
        <ULID as Value>::value(&self.value)
    }
}

/// Represents the registered user returned by the use case.
///
/// The registered user is the successful bounded response for the example use
/// case, so no separate response wrapper is needed.
#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisteredUser {
    id: UserId,
    email: EmailAddress,
}

impl RegisteredUser {
    /// Creates a [`RegisteredUser`] from a validated identity and email address.
    ///
    /// # Arguments
    ///
    /// * `id` - The stable identity for the registered user entity.
    /// * `email` - The validated email address for the registered user.
    fn new(id: UserId, email: EmailAddress) -> Self {
        Self { id, email }
    }

    /// Returns the registered user's email address.
    fn email(&self) -> &EmailAddress {
        &self.email
    }
}

impl Entity for RegisteredUser {
    type IdType = UserId;

    /// Returns the stable registered user identity.
    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

/// Represents the built request accepted by [`RegisterUserUC`].
///
/// This is a non-void request object, so callers construct it with
/// [`RegisterUserRequestBuilder`] before calling the use case.
#[derive(Debug)]
struct RegisterUserRequest {
    email: EmailAddress,
}

impl RegisterUserRequest {
    /// Consumes the request and returns the validated email address.
    fn into_email(self) -> EmailAddress {
        self.email
    }
}

/// Builds a [`RegisterUserRequest`] for the register-user use case seam.
///
/// The builder owns construction of the non-void request object. The use case
/// receives only the built request.
#[derive(Debug, Default)]
struct RegisterUserRequestBuilder {
    email: Option<EmailAddress>,
}

impl RegisterUserRequestBuilder {
    /// Returns a new [`RegisterUserRequestBuilder`].
    fn new() -> Self {
        Self::default()
    }

    /// Sets the validated email address for the request.
    ///
    /// # Arguments
    ///
    /// * `email` - The validated email address required by the request.
    ///
    /// # Returns
    ///
    /// Returns the builder with the email address set.
    fn with_email(mut self, email: EmailAddress) -> Self {
        self.email = Some(email);
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
    fn try_build(self) -> Result<RegisterUserRequest, Error> {
        let email = self.email.ok_or_else(|| {
            Error::for_user(Kind::InvalidInput, "register user email is required")
        })?;

        Ok(RegisterUserRequest { email })
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
/// composes the successful [`RegisteredUser`] response.
struct RegisterUserUseCase {
    new_identity_gateway: Arc<dyn NewIdentityGW>,
}

impl RegisterUserUseCase {
    /// Creates a [`RegisterUserUseCase`] with the required gateway dependency.
    ///
    /// # Arguments
    ///
    /// * `new_identity_gateway` - The gateway seam used to request new user identities.
    fn new(new_identity_gateway: Arc<dyn NewIdentityGW>) -> Self {
        Self {
            new_identity_gateway,
        }
    }
}

impl UseCase for RegisterUserUseCase {
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
        let identity = self
            .new_identity_gateway
            .execute(NewIdentityGatewayRequest)?;

        Ok(RegisteredUser::new(
            UserId::new(identity),
            request.into_email(),
        ))
    }
}

impl RegisterUserUC for RegisterUserUseCase {}

/// Provides a deterministic new-identity gateway for the example.
///
/// This gateway implements [`NewIdentityGW`] by returning a configured
/// [`ULID`], which keeps the example deterministic.
struct StaticNewIdentityGateway {
    identity: ULID,
}

impl StaticNewIdentityGateway {
    /// Creates a [`StaticNewIdentityGateway`] that returns the provided identity.
    ///
    /// # Arguments
    ///
    /// * `identity` - The [`ULID`] returned by the gateway.
    fn new(identity: ULID) -> Self {
        Self { identity }
    }
}

impl Gateway for StaticNewIdentityGateway {
    type Request = NewIdentityGatewayRequest;
    type Response = ULID;

    /// Executes the new-identity gateway with a named void request object.
    ///
    /// # Arguments
    ///
    /// * `_request` - The explicit void-by-construction gateway request object.
    ///
    /// # Returns
    ///
    /// Returns the configured [`ULID`] directly as the bounded response.
    ///
    /// # Errors
    ///
    /// This deterministic example implementation does not return an error.
    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
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
    let new_identity_gateway = Arc::new(StaticNewIdentityGateway::new(generated_identity));
    let use_case: Box<dyn RegisterUserUC> =
        Box::new(RegisterUserUseCase::new(new_identity_gateway));

    let request = RegisterUserRequestBuilder::new()
        .with_email(EmailAddress::try_new("OWNER@example.com")?)
        .try_build()?;

    let user = use_case.execute(request)?;

    assert_eq!(user.email().value(), "owner@example.com");
    assert_eq!(
        user.id().value(),
        <ULID as Value>::value(&generated_identity)
    );

    Ok(())
}
