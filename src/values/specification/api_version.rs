//! API version value for specification models.

use crate::error::{Error, Kind};
use std::str::FromStr;

/// [`APIVersion`] represents the version of the NAPE API versioning primarily for the NAPE Specifications and follows the [Semantic Versioning 2.0.0 specification](https://github.com/semver/semver)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct APIVersion {
    /// Major version component.
    pub major: u8,
    /// Minor version component.
    pub minor: u8,
    /// Patch version component.
    pub patch: u8,
}

impl APIVersion {
    /// Returns the major version number.
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Returns the patch version number.
    pub fn patch(&self) -> u8 {
        self.patch
    }

    /// Create a new [`APIVersion`] instance
    ///
    /// # Arguments
    ///
    /// * `major` - The major version number
    /// * `minor` - The minor version number
    /// * `patch` - The patch version number
    ///
    /// # Returns
    ///
    /// A new [`APIVersion`] instance
    ///
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        APIVersion {
            major,
            minor,
            patch,
        }
    }

    /// Get the [`APIVersion`] as a string.
    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for APIVersion {
    type Err = Error;

    /// Parses an [`APIVersion`] from a string in `major.minor.patch` form.
    fn from_str(version: &str) -> Result<Self, Self::Err> {
        if version.trim().is_empty() {
            return Err(invalid_input_error("You provided an empty version. Please provide a version value which conforms to the Semver specification of 'major.minor.patch'.".to_string()));
        }

        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(invalid_input_error(format!(
                "Version string '{}' is not in the format 'major.minor.patch'.",
                version
            )));
        }

        let major = parts[0].parse::<u8>().map_err(|_| invalid_input_error(format!("The major version '{}' from the supplied api version of '{}' is not a valid number. It must be a number between 0 and 255.", parts[0], version)) )?;
        let minor = parts[1].parse::<u8>().map_err(|_| invalid_input_error(format!("The minor version '{}' from the supplied api version of '{}' is not a valid number.  It must be a number between 0 and 255", parts[1], version)) )?;
        let patch = parts[2].parse::<u8>().map_err(|_| invalid_input_error(format!("The patch version '{}' from the supplied api version of '{}' is not a valid number. It must be a number between 0 and 255", parts[2], version)) )?;

        Ok(APIVersion {
            major,
            minor,
            patch,
        })
    }
}

fn invalid_input_error(message: String) -> Error {
    Error::for_user(Kind::InvalidInput, message)
}
