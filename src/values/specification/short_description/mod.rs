//! Specification short-description value.

use crate::error::{Error, Kind};
use crate::values::Value;
use crate::values::specification::description::Description;
use crate::values::strings::exceeds_max_length;

/// the [`MAX_LENGTH`] is the maximum amount of characters the [`ShortDescription`] can contain
const MAX_LENGTH: usize = 255;

/// The [`ShortDescription`] struct represents a short description of something.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortDescription {
    /// Canonical short description text.
    pub value: String,
}

impl ShortDescription {
    /// Returns the short description value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Creates a new [`ShortDescription`] instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that holds the value of the short description.
    ///
    /// # Returns
    ///
    /// A Result containing either a [`ShortDescription`] instance or an [`Error`].
    ///
    /// ## Errors
    ///
    /// An [`Error`] of [`Kind::InvalidInput`] for [`Audience::User`](crate::error::Audience::User) will be returned if:
    /// * the value is empty, or
    /// * if the value exceeds the maximum length of 255 characters
    ///
    pub fn try_from(value: &str) -> Result<Self, Error> {
        let desc = Description::try_from(value).map_err(|e| customize_error(e.message.as_str()))?;
        if exceeds_max_length(desc.value.as_str(), MAX_LENGTH) {
            return Err(customize_error(
                format!("The short description provided is too long. A description must have a value with at most {} characters.", MAX_LENGTH).as_str()));
        }
        Ok(Self { value: desc.value })
    }
}

impl Value for ShortDescription {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

fn customize_error(message: &str) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!("There is an issue with your short description: {}", message),
    )
}

#[cfg(test)]
mod tests;
