//! Demonstrates a standards-aligned value object and entity.
//!
//! This example shows how a value object implements [`Value`] with a
//! fundamental Rust value type and how an entity implements [`Entity`] with a
//! bounded identity value object.
//!
//! Run this example with:
//!
//! ```text
//! cargo run --example value_object_and_entity
//! ```

use kernel_oss::core::traits::Entity;
use kernel_oss::error::{Error, Kind};
use kernel_oss::ulid::ULID;
use kernel_oss::values::Value;

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

/// Represents a user entity identified by [`UserId`].
///
/// The entity owns its identity and email address. Its stable identity is
/// exposed through the shared [`Entity`] trait.
#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    id: UserId,
    email: EmailAddress,
}

impl User {
    /// Creates a [`User`] from a validated identity and email address.
    ///
    /// # Arguments
    ///
    /// * `id` - The stable identity for the user entity.
    /// * `email` - The validated email address for the user.
    fn new(id: UserId, email: EmailAddress) -> Self {
        Self { id, email }
    }

    /// Returns the user's email address.
    fn email(&self) -> &EmailAddress {
        &self.email
    }
}

impl Entity for User {
    type IdType = UserId;

    /// Returns the stable user identity.
    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

/// Runs the value object and entity example.
///
/// # Errors
///
/// Returns an error if:
/// - the example email address is empty or contains only whitespace
/// - the example email address does not contain `@`
fn main() -> Result<(), Error> {
    let id = UserId::new(ULID::from_parts(1, 7));
    let email = EmailAddress::try_new("OWNER@example.com")?;
    let user = User::new(id, email);

    assert_eq!(user.email().value(), "owner@example.com");
    assert_eq!(user.id().value(), id.value());

    Ok(())
}
