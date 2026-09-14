//! External-consumer evidence for the public bounded Error provenance API.
//!
//! This integration test imports only `kernel_oss::error` public re-exports.
//! Its local definition set demonstrates the permitted ABI-level construction
//! shape for a generic semantic owner; it does not use a private Kernel helper,
//! a test-only alias, or a test-configuration implementation path.

use kernel_oss::error::{
    Audience, Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

const ATTACH_PRESENT_MESSAGE: &str =
    "Error provenance cannot be attached because it is already present.";
const DERIVE_ABSENT_MESSAGE: &str = "Error provenance cannot be derived because it is absent.";
const DERIVE_ORIGIN_MESSAGE: &str =
    "Error provenance cannot be derived because an origin is already present.";

struct GenericProvenanceDefinitions {
    gateway_failure: ErrorProvenanceDescriptor,
    application_failure: ErrorProvenanceDescriptor,
    recovery_failure: ErrorProvenanceDescriptor,
}

impl GenericProvenanceDefinitions {
    fn try_new() -> Result<Self, Error> {
        let namespace = ErrorProvenanceNamespace::try_new("example.owner")?;
        let schema_version = ErrorProvenanceSchemaVersion::try_new(1)?;
        Ok(Self {
            gateway_failure: ErrorProvenanceDescriptor::new(
                namespace.clone(),
                schema_version,
                ErrorProvenanceCode::try_new("gateway_failure")?,
            ),
            application_failure: ErrorProvenanceDescriptor::new(
                namespace.clone(),
                schema_version,
                ErrorProvenanceCode::try_new("application_failure")?,
            ),
            recovery_failure: ErrorProvenanceDescriptor::new(
                namespace,
                schema_version,
                ErrorProvenanceCode::try_new("recovery_failure")?,
            ),
        })
    }

    fn gateway_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.gateway_failure
    }

    fn application_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.application_failure
    }

    fn recovery_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.recovery_failure
    }
}

/// Requirement validation: D2-ABI-01.
///
/// Public seam: `kernel_oss::error` re-exports and Error reconstruction
/// methods only.
/// Logical path: a generic external owner constructs its immutable fallible
/// definition set, then attaches, derives, inspects, compares, and clears.
/// Observable result: each transition returns complete public identity while
/// every source Error remains unchanged.
#[test]
fn public_error_provenance_transitions_preserve_each_source_success() {
    let definitions = match GenericProvenanceDefinitions::try_new() {
        Ok(value) => value,
        Err(error) => panic!("the generic public definition set must construct: {error:?}"),
    };
    let original = Error::new(
        Audience::System,
        Kind::GatewayError,
        "Dependency is unavailable.",
    );
    let original_before_attach = original.clone();
    let attached =
        match original.try_with_attached_provenance(definitions.gateway_failure().clone()) {
            Ok(value) => value,
            Err(error) => panic!("public attachment must succeed: {error:?}"),
        };
    assert_eq!(original, original_before_attach);
    assert!(original.provenance().is_none());

    let attached_before_derive = attached.clone();
    let derived =
        match attached.try_with_derived_provenance(definitions.application_failure().clone()) {
            Ok(value) => value,
            Err(error) => panic!("public derivation must succeed: {error:?}"),
        };
    assert_eq!(attached, attached_before_derive);
    let attached_provenance = match attached.provenance() {
        Some(value) => value,
        None => panic!("attached Error must retain public provenance"),
    };
    assert_eq!(attached_provenance.current(), definitions.gateway_failure());
    assert_eq!(attached_provenance.origin(), None);

    let derived_before_clear = derived.clone();
    let provenance = match derived.provenance() {
        Some(value) => value,
        None => panic!("derived Error must retain public provenance"),
    };
    assert_eq!(provenance.current(), definitions.application_failure());
    assert_eq!(provenance.origin(), Some(definitions.gateway_failure()));
    assert!(derived.has_current_provenance(definitions.application_failure()));
    assert_ne!(derived, attached);

    let cleared = derived.without_provenance();
    assert_eq!(derived, derived_before_clear);
    assert!(cleared.provenance().is_none());
    assert_eq!(cleared.audience(), Audience::System);
    assert_eq!(cleared.kind(), Kind::GatewayError);
    assert_eq!(cleared.message(), "Dependency is unavailable.");
}

/// Requirement validation: D2-ABI-02.
///
/// Public seam: `Error::try_with_attached_provenance`.
/// Logical path: an external caller attempts a second attachment.
/// Observable result: the exact bounded Error is returned and the source Error
/// retains its original current-only provenance.
#[test]
fn public_error_attach_when_present_returns_exact_error_and_preserves_source_error() {
    let definitions = match GenericProvenanceDefinitions::try_new() {
        Ok(value) => value,
        Err(error) => panic!("the generic public definition set must construct: {error:?}"),
    };
    let attached = match Error::for_system(Kind::GatewayError, "Dependency is unavailable.")
        .try_with_attached_provenance(definitions.gateway_failure().clone())
    {
        Ok(value) => value,
        Err(error) => panic!("public fixture attachment must succeed: {error:?}"),
    };
    let source_before_rejection = attached.clone();
    let result = attached.try_with_attached_provenance(definitions.application_failure().clone());
    let error = match result {
        Ok(_) => panic!("a second public attachment must fail"),
        Err(error) => error,
    };
    assert_eq!(error.audience(), Audience::System);
    assert_eq!(error.kind(), Kind::ProcessingFailure);
    assert_eq!(error.message(), ATTACH_PRESENT_MESSAGE);
    assert!(error.provenance().is_none());
    assert_eq!(attached, source_before_rejection);
}

/// Requirement validation: D2-ABI-03.
///
/// Public seam: `Error::try_with_derived_provenance`.
/// Logical path: an external caller derives from an Error with absent provenance.
/// Observable result: the exact bounded Error is returned and the source Error
/// remains provenance-free.
#[test]
fn public_error_derive_when_absent_returns_exact_error_and_preserves_source_error() {
    let definitions = match GenericProvenanceDefinitions::try_new() {
        Ok(value) => value,
        Err(error) => panic!("the generic public definition set must construct: {error:?}"),
    };
    let source = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    let source_before_rejection = source.clone();
    let result = source.try_with_derived_provenance(definitions.application_failure().clone());
    let error = match result {
        Ok(_) => panic!("public derivation without provenance must fail"),
        Err(error) => error,
    };
    assert_eq!(error.audience(), Audience::System);
    assert_eq!(error.kind(), Kind::ProcessingFailure);
    assert_eq!(error.message(), DERIVE_ABSENT_MESSAGE);
    assert!(error.provenance().is_none());
    assert_eq!(source, source_before_rejection);
}

/// Requirement validation: D2-ABI-04.
///
/// Public seam: `Error::try_with_derived_provenance`.
/// Logical path: an external caller attempts a second derivation after an
/// origin descriptor already exists.
/// Observable result: the exact bounded Error is returned and the source Error
/// retains its complete current-plus-origin provenance identity.
#[test]
fn public_error_derive_when_origin_present_returns_exact_error_and_preserves_source_error() {
    let definitions = match GenericProvenanceDefinitions::try_new() {
        Ok(value) => value,
        Err(error) => panic!("the generic public definition set must construct: {error:?}"),
    };
    let attached = match Error::for_system(Kind::GatewayError, "Dependency is unavailable.")
        .try_with_attached_provenance(definitions.gateway_failure().clone())
    {
        Ok(value) => value,
        Err(error) => panic!("public fixture attachment must succeed: {error:?}"),
    };
    let derived =
        match attached.try_with_derived_provenance(definitions.application_failure().clone()) {
            Ok(value) => value,
            Err(error) => panic!("public fixture derivation must succeed: {error:?}"),
        };
    let source_before_rejection = derived.clone();
    let result = derived.try_with_derived_provenance(definitions.recovery_failure().clone());
    let error = match result {
        Ok(_) => panic!("a second public derivation must fail"),
        Err(error) => error,
    };
    assert_eq!(error.audience(), Audience::System);
    assert_eq!(error.kind(), Kind::ProcessingFailure);
    assert_eq!(error.message(), DERIVE_ORIGIN_MESSAGE);
    assert!(error.provenance().is_none());
    assert_eq!(derived, source_before_rejection);
}
