//! Specification file path value.

use crate::error::{Error, Kind};
use crate::values::Value;
use std::fmt::{Display, Formatter};

/// Bounded specification file path text.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct FilePath {
    /// Canonical file path text.
    value: String,
}

impl FilePath {
    /// Creates a new instance of a [`FilePath`] without any validations.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice containing the file path.
    ///
    /// # Returns
    ///
    /// A new instance of a [`FilePath`].
    pub fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    /// Creates a new instance of a [`FilePath`] and applies validations to the arguments.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice containing the file path.
    ///
    /// # Returns
    ///
    /// A new instance of a [`FilePath`], or an [`Error`] if the value is:
    /// * Empty
    pub fn try_from(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The file path is required, although a value was not provided.".to_string(),
            ));
        }
        Ok(Self {
            value: value.to_string(),
        })
    }

    /// Returns the path as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

impl Value for FilePath {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

#[cfg(test)]
mod tests;
