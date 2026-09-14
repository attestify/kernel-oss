//! Immutable bounded descriptors carried by [`super::Error`].
//!
//! Provenance represents one current semantic descriptor and, after one
//! permitted derivation, one origin descriptor. It never carries request data,
//! HTTP values, native diagnostics, arbitrary metadata, or dynamic payloads.

use std::num::NonZeroU16;

use super::{Error, Kind};

const INVALID_NAMESPACE_MESSAGE: &str = "Error provenance namespace is invalid.";
const INVALID_CODE_MESSAGE: &str = "Error provenance code is invalid.";
const INVALID_SCHEMA_VERSION_MESSAGE: &str = "Error provenance schema version must be nonzero.";

/// A bounded lowercase dot-separated namespace for one error semantic owner.
///
/// A namespace contains two through eight ASCII segments and is three through
/// 128 bytes long. Each segment starts with a lowercase ASCII letter and may
/// then contain lowercase ASCII letters, digits, or hyphens.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorProvenanceNamespace(String);

impl ErrorProvenanceNamespace {
    /// Validates and constructs a provenance namespace without normalization.
    ///
    /// # Errors
    ///
    /// Returns a system invalid-input error when the grammar or bounds are not
    /// satisfied.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceNamespace;
    ///
    /// let namespace = ErrorProvenanceNamespace::try_new("attestify.kernel")
    ///     .expect("the namespace is valid");
    /// assert_eq!(namespace.as_str(), "attestify.kernel");
    /// ```
    pub fn try_new(value: impl AsRef<str>) -> Result<Self, Error> {
        let value = value.as_ref();
        if !namespace_is_valid(value) {
            return Err(invalid_namespace_error());
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the exact validated namespace text.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceNamespace;
    ///
    /// let namespace = ErrorProvenanceNamespace::try_new("attestify.kernel")
    ///     .expect("the namespace is valid");
    /// assert_eq!(namespace.as_str(), "attestify.kernel");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A bounded lowercase identifier for one documented error semantic.
///
/// A code is one through 64 ASCII bytes, starts with a lowercase ASCII letter,
/// and subsequently permits lowercase ASCII letters, digits, and underscores.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorProvenanceCode(String);

impl ErrorProvenanceCode {
    /// Validates and constructs a provenance code without normalization.
    ///
    /// # Errors
    ///
    /// Returns a system invalid-input error when the grammar or bounds are not
    /// satisfied.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceCode;
    ///
    /// let code = ErrorProvenanceCode::try_new("dependency_failed")
    ///     .expect("the code is valid");
    /// assert_eq!(code.as_str(), "dependency_failed");
    /// ```
    pub fn try_new(value: impl AsRef<str>) -> Result<Self, Error> {
        let value = value.as_ref();
        if !code_is_valid(value) {
            return Err(invalid_code_error());
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the exact validated code text.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceCode;
    ///
    /// let code = ErrorProvenanceCode::try_new("dependency_failed")
    ///     .expect("the code is valid");
    /// assert_eq!(code.as_str(), "dependency_failed");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A nonzero schema version for a provenance descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ErrorProvenanceSchemaVersion(NonZeroU16);

impl ErrorProvenanceSchemaVersion {
    /// Validates and constructs a nonzero provenance schema version.
    ///
    /// # Errors
    ///
    /// Returns a system invalid-input error when `value` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceSchemaVersion;
    ///
    /// let version = ErrorProvenanceSchemaVersion::try_new(1)
    ///     .expect("the version is nonzero");
    /// assert_eq!(version.get(), 1);
    /// ```
    pub fn try_new(value: u16) -> Result<Self, Error> {
        NonZeroU16::new(value)
            .map(Self)
            .ok_or_else(invalid_schema_version_error)
    }

    /// Returns the exact nonzero schema version as a `u16`.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::ErrorProvenanceSchemaVersion;
    ///
    /// let version = ErrorProvenanceSchemaVersion::try_new(u16::MAX)
    ///     .expect("the maximum u16 is nonzero");
    /// assert_eq!(version.get(), u16::MAX);
    /// ```
    pub fn get(&self) -> u16 {
        self.0.get()
    }
}

/// A complete immutable provenance descriptor.
///
/// The descriptor has no cross-field invariant because each input is already a
/// validated bounded value. Construction is therefore intentionally infallible.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorProvenanceDescriptor {
    namespace: ErrorProvenanceNamespace,
    schema_version: ErrorProvenanceSchemaVersion,
    code: ErrorProvenanceCode,
}

impl ErrorProvenanceDescriptor {
    /// Constructs a descriptor from valid bounded values.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion,
    /// };
    ///
    /// let descriptor = ErrorProvenanceDescriptor::new(
    ///     ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ///     ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ///     ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
    /// );
    /// assert_eq!(descriptor.code().as_str(), "dependency_failed");
    /// ```
    pub fn new(
        namespace: ErrorProvenanceNamespace,
        schema_version: ErrorProvenanceSchemaVersion,
        code: ErrorProvenanceCode,
    ) -> Self {
        Self {
            namespace,
            schema_version,
            code,
        }
    }

    /// Returns the descriptor namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion,
    /// };
    ///
    /// let descriptor = ErrorProvenanceDescriptor::new(
    ///     ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ///     ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ///     ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
    /// );
    /// assert_eq!(descriptor.namespace().as_str(), "attestify.kernel");
    /// ```
    pub fn namespace(&self) -> &ErrorProvenanceNamespace {
        &self.namespace
    }

    /// Returns the descriptor schema version.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion,
    /// };
    ///
    /// let descriptor = ErrorProvenanceDescriptor::new(
    ///     ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ///     ErrorProvenanceSchemaVersion::try_new(2).expect("nonzero version"),
    ///     ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
    /// );
    /// assert_eq!(descriptor.schema_version().get(), 2);
    /// ```
    pub fn schema_version(&self) -> ErrorProvenanceSchemaVersion {
        self.schema_version
    }

    /// Returns the descriptor code.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion,
    /// };
    ///
    /// let descriptor = ErrorProvenanceDescriptor::new(
    ///     ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ///     ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ///     ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
    /// );
    /// assert_eq!(descriptor.code().as_str(), "dependency_failed");
    /// ```
    pub fn code(&self) -> &ErrorProvenanceCode {
        &self.code
    }
}

/// Immutable current and optional origin provenance for an [`Error`].
///
/// Only [`Error::try_with_attached_provenance`] and
/// [`Error::try_with_derived_provenance`] construct provenance, so no public
/// caller can make an invalid chain or more than one origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorProvenance {
    current: ErrorProvenanceDescriptor,
    origin: Option<ErrorProvenanceDescriptor>,
}

impl ErrorProvenance {
    pub(super) fn new(
        current: ErrorProvenanceDescriptor,
        origin: Option<ErrorProvenanceDescriptor>,
    ) -> Self {
        Self { current, origin }
    }

    /// Returns the current provenance descriptor.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion, Kind,
    /// };
    ///
    /// let descriptor = ErrorProvenanceDescriptor::new(
    ///     ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ///     ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ///     ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
    /// );
    /// let error = Error::for_system(Kind::GatewayError, "Unavailable.")
    ///     .try_with_attached_provenance(descriptor.clone())
    ///     .expect("provenance is absent");
    /// assert_eq!(error.provenance().expect("attached provenance").current(), &descriptor);
    /// ```
    pub fn current(&self) -> &ErrorProvenanceDescriptor {
        &self.current
    }

    /// Returns the former current descriptor after one derivation, if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{
    ///     Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ///     ErrorProvenanceSchemaVersion, Kind,
    /// };
    ///
    /// let namespace = ErrorProvenanceNamespace::try_new("attestify.kernel")
    ///     .expect("valid namespace");
    /// let version = ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version");
    /// let current = ErrorProvenanceDescriptor::new(
    ///     namespace.clone(), version, ErrorProvenanceCode::try_new("gateway_failed").expect("valid code"),
    /// );
    /// let derived = ErrorProvenanceDescriptor::new(
    ///     namespace, version, ErrorProvenanceCode::try_new("application_failed").expect("valid code"),
    /// );
    /// let error = Error::for_system(Kind::GatewayError, "Unavailable.")
    ///     .try_with_attached_provenance(current.clone())
    ///     .expect("provenance is absent")
    ///     .try_with_derived_provenance(derived)
    ///     .expect("only current provenance is present");
    /// assert_eq!(error.provenance().expect("derived provenance").origin(), Some(&current));
    /// ```
    pub fn origin(&self) -> Option<&ErrorProvenanceDescriptor> {
        self.origin.as_ref()
    }
}

fn namespace_is_valid(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !(3..=128).contains(&bytes.len()) || !bytes.is_ascii() {
        return false;
    }
    let segments: Vec<&str> = value.split('.').collect();
    if !(2..=8).contains(&segments.len()) {
        return false;
    }
    segments.iter().all(|segment| segment_is_valid(segment))
}

fn segment_is_valid(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    (1..=32).contains(&bytes.len())
        && bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes[1..]
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}

fn code_is_valid(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=64).contains(&bytes.len())
        && bytes.is_ascii()
        && bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes[1..]
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
}

fn invalid_namespace_error() -> Error {
    Error::for_system(Kind::InvalidInput, INVALID_NAMESPACE_MESSAGE)
}

fn invalid_code_error() -> Error {
    Error::for_system(Kind::InvalidInput, INVALID_CODE_MESSAGE)
}

fn invalid_schema_version_error() -> Error {
    Error::for_system(Kind::InvalidInput, INVALID_SCHEMA_VERSION_MESSAGE)
}
