//! Bounded error classifications and immutable provenance for kernel seams.
//!
//! [`Error`] remains the failure value in `Result<Response, Error>` APIs. Its
//! optional provenance is a bounded semantic descriptor, not ambient metadata,
//! a diagnostic payload, or a source-error chain.

pub mod provenance;

pub use provenance::{
    ErrorProvenance, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion,
};

#[cfg(test)]
mod tests;

const ATTACH_PRESENT_MESSAGE: &str =
    "Error provenance cannot be attached because it is already present.";
const DERIVE_ABSENT_MESSAGE: &str = "Error provenance cannot be derived because it is absent.";
const DERIVE_ORIGIN_MESSAGE: &str =
    "Error provenance cannot be derived because an origin is already present.";

/// A structured, bounded failure returned by public kernel APIs.
///
/// The outer audience, kind, and message retain their existing behavior.
/// Provenance is optional, immutable semantic classification with at most one
/// origin descriptor. It is intentionally absent from newly constructed
/// errors and can only be changed through reconstruction methods.
///
/// `Error` implements complete equality over its outer fields and provenance,
/// but intentionally does not implement `Hash`: provenance is semantic state
/// and must never be silently omitted from a hash identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// Audience classification for the error.
    pub audience: Audience,
    /// Coarse error category.
    pub kind: Kind,
    /// Human-readable error message.
    pub message: String,
    provenance: Option<Box<ErrorProvenance>>,
}

impl Error {
    /// Creates a bounded error with no provenance.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Audience, Error, Kind};
    ///
    /// let error = Error::new(Audience::User, Kind::InvalidInput, "Field is required.");
    /// assert_eq!(error.audience(), Audience::User);
    /// assert!(error.provenance().is_none());
    /// ```
    pub fn new(audience: Audience, kind: Kind, message: impl Into<String>) -> Self {
        Self {
            audience,
            kind,
            message: message.into(),
            provenance: None,
        }
    }

    /// Creates a user-audience error with no provenance.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// let error = Error::for_user(Kind::InvalidInput, "Field is required.");
    /// assert!(error.is_user());
    /// assert!(error.provenance().is_none());
    /// ```
    pub fn for_user(kind: Kind, message: impl Into<String>) -> Self {
        Self::new(Audience::User, kind, message)
    }

    /// Creates a system-audience error with no provenance.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    /// assert!(error.is_system());
    /// assert!(error.provenance().is_none());
    /// ```
    pub fn for_system(kind: Kind, message: impl Into<String>) -> Self {
        Self::new(Audience::System, kind, message)
    }

    /// Returns the audience that should handle this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Audience, Error, Kind};
    ///
    /// assert_eq!(Error::for_user(Kind::NotFound, "Missing.").audience(), Audience::User);
    /// ```
    pub fn audience(&self) -> Audience {
        self.audience
    }

    /// Returns the coarse category for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// assert_eq!(Error::for_system(Kind::GatewayError, "Unavailable.").kind(), Kind::GatewayError);
    /// ```
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns the stable human-readable message.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// assert_eq!(Error::for_user(Kind::InvalidInput, "Invalid.").message(), "Invalid.");
    /// ```
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns whether this error is intended for a user audience.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// assert!(Error::for_user(Kind::InvalidInput, "Invalid.").is_user());
    /// ```
    pub fn is_user(&self) -> bool {
        self.audience == Audience::User
    }

    /// Returns whether this error is intended for system or operational handling.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// assert!(Error::for_system(Kind::GatewayError, "Unavailable.").is_system());
    /// ```
    pub fn is_system(&self) -> bool {
        self.audience == Audience::System
    }

    /// Returns the immutable provenance, if a semantic owner attached one.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// let error = Error::for_system(Kind::ProcessingFailure, "Failed.");
    /// assert!(error.provenance().is_none());
    /// ```
    pub fn provenance(&self) -> Option<&ErrorProvenance> {
        self.provenance.as_deref()
    }

    /// Returns whether the current provenance equals `descriptor`.
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
    /// assert!(error.has_current_provenance(&descriptor));
    /// ```
    pub fn has_current_provenance(&self, descriptor: &ErrorProvenanceDescriptor) -> bool {
        self.provenance
            .as_ref()
            .is_some_and(|provenance| provenance.current() == descriptor)
    }

    /// Reconstructs this error with `descriptor` as its current provenance.
    ///
    /// Attachment is allowed only while provenance is absent. A failed
    /// transition returns an exact bounded error and leaves this borrowed error
    /// unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - this Error already has provenance.
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
    /// let attached = Error::for_system(Kind::GatewayError, "Unavailable.")
    ///     .try_with_attached_provenance(descriptor.clone())
    ///     .expect("provenance is absent");
    /// assert_eq!(attached.provenance().expect("attached provenance").current(), &descriptor);
    /// ```
    pub fn try_with_attached_provenance(
        &self,
        descriptor: ErrorProvenanceDescriptor,
    ) -> Result<Self, Error> {
        if self.provenance.is_some() {
            return Err(Self::for_system(
                Kind::ProcessingFailure,
                ATTACH_PRESENT_MESSAGE,
            ));
        }
        Ok(self.reconstruct(Some(ErrorProvenance::new(descriptor, None))))
    }

    /// Reconstructs this error by making `descriptor` current and preserving
    /// the former current descriptor as its one origin.
    ///
    /// Derivation is allowed only when exactly one current descriptor exists.
    /// A failed transition returns an exact bounded error and leaves this
    /// borrowed error unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - this Error has no current provenance; or
    /// - its current provenance already has an origin descriptor.
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
    ///     .try_with_derived_provenance(derived.clone())
    ///     .expect("only current provenance is present");
    /// let provenance = error.provenance().expect("derived provenance");
    /// assert_eq!(provenance.current(), &derived);
    /// assert_eq!(provenance.origin(), Some(&current));
    /// ```
    pub fn try_with_derived_provenance(
        &self,
        descriptor: ErrorProvenanceDescriptor,
    ) -> Result<Self, Error> {
        let provenance = match self.provenance.as_ref() {
            None => {
                return Err(Self::for_system(
                    Kind::ProcessingFailure,
                    DERIVE_ABSENT_MESSAGE,
                ));
            }
            Some(provenance) if provenance.origin().is_some() => {
                return Err(Self::for_system(
                    Kind::ProcessingFailure,
                    DERIVE_ORIGIN_MESSAGE,
                ));
            }
            Some(provenance) => provenance,
        };
        Ok(self.reconstruct(Some(ErrorProvenance::new(
            descriptor,
            Some(provenance.current().clone()),
        ))))
    }

    /// Reconstructs this error with all provenance removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::error::{Error, Kind};
    ///
    /// let cleared = Error::for_system(Kind::ProcessingFailure, "Failed.").without_provenance();
    /// assert!(cleared.provenance().is_none());
    /// ```
    pub fn without_provenance(&self) -> Self {
        self.reconstruct(None)
    }

    fn reconstruct(&self, provenance: Option<ErrorProvenance>) -> Self {
        Self {
            audience: self.audience,
            kind: self.kind,
            message: self.message.clone(),
            provenance: provenance.map(Box::new),
        }
    }
}

impl std::fmt::Display for Error {
    /// Displays only the human-readable message.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

/// Specifies who should handle an [`Error`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Audience {
    /// Intended for end users.
    User,
    /// Intended for system or operational handling.
    System,
}

/// High-level categories for [`Error`] instances.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    /// A value exceeds the maximum allowed length.
    ExceedsMax,
    /// A value is below the minimum allowed length.
    BelowMin,
    /// A requested item was not found.
    NotFound,
    /// The supplied input does not meet the required constraints.
    InvalidInput,
    /// A catch-all for unexpected cases.
    Unexpected,
    /// A gateway execution error occurred.
    GatewayError,
    /// A use case execution error occurred.
    UsecaseError,
    /// Permission was denied.
    PermissionDenied,
    /// A local processing failure occurred.
    ProcessingFailure,
}
