//! Demonstrates a standards-aligned value object and entity.
//!
//! This example shows how a value object implements [`Value`] with a
//! fundamental Rust value type and how an entity implements [`Entity`] with a
//! direct [`ULID`] identity.
//!
//! Run this example with:
//!
//! ```text
//! cargo run --example value_object_and_entity
//! ```

use kernel_oss::entity::Entity;
use kernel_oss::error::{Error, Kind};
use kernel_oss::ulid::ULID;
use kernel_oss::values::Value;
use std::fmt;

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

/// Represents a user entity identified directly by [`ULID`].
///
/// The entity owns its identity and email address. Its stable identity is
/// exposed through the shared [`Entity`] trait. Equality is identity-based.
#[derive(Clone, Debug)]
struct User {
    id: ULID,
    email: EmailAddress,
}

impl User {
    /// Creates a [`User`] from a validated identity and email address.
    ///
    /// # Arguments
    ///
    /// * `id` - The stable identity for the user entity.
    /// * `email` - The validated email address for the user.
    fn new(id: ULID, email: EmailAddress) -> Self {
        Self { id, email }
    }

    /// Returns the user's email address.
    fn email(&self) -> &EmailAddress {
        &self.email
    }
}

impl Entity for User {
    type IdType = ULID;

    /// Returns the stable user identity.
    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

impl PartialEq for User {
    /// Compares users by stable identity.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

/// Runs the value object and entity example.
///
/// # Errors
///
/// Returns an error if:
/// - the example email address is empty or contains only whitespace
/// - the example email address does not contain `@`
fn main() -> Result<(), Error> {
    let id = ULID::from_parts(1, 7);
    let email = EmailAddress::try_new("OWNER@example.com")?;
    let user = User::new(id, email);
    let changed_email = EmailAddress::try_new("new-owner@example.com")?;
    let same_user = User::new(id, changed_email);

    assert_eq!(user.email().value(), "owner@example.com");
    assert_eq!(user.id(), &id);
    assert_eq!(user.id().value(), id.value());
    assert_eq!(user, same_user);

    Ok(())
}
