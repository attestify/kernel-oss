//! Specification subject identifier value.

use crate::error::{Error, Kind};
use crate::values::Value;
use crate::values::strings::{STRING_256_MAX, exceeds_max_length, has_more_than_alphanumeric};

/// The `SubjectId` is a unique identifier for the subject of an NAPE entity.  It can only be alphanumeric characters and is limited to 256 characters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SubjectId {
    /// Canonical subject identifier text.
    pub value: String,
}

impl SubjectId {
    /// Returns the subject identifier value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Creates a bounded subject identifier.
    pub fn new(value: &str) -> Result<SubjectId, Error> {
        if value.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The Subject Id cannot be empty.".to_string(),
            ));
        }
        if exceeds_max_length(value, STRING_256_MAX) {
            return Err(Error::for_user(
                Kind::InvalidInput,
                format!(
                    "The SubjectId exceeds the maximum length of {} characters.",
                    STRING_256_MAX
                ),
            ));
        }
        if has_more_than_alphanumeric(value) {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The SubjectId can only contain alphanumeric characters.".to_string(),
            ));
        }

        Ok(SubjectId {
            value: value.to_string(),
        })
    }
}

impl Value for SubjectId {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

#[cfg(test)]
mod tests;
