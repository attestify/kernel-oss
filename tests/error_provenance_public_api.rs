//! External-consumer evidence for the public bounded Error provenance API.

use kernel_oss::error::{
    Audience, Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

fn descriptor(code: &str) -> ErrorProvenanceDescriptor {
    let namespace = match ErrorProvenanceNamespace::try_new("attestify.kernel") {
        Ok(value) => value,
        Err(error) => panic!("the public namespace fixture must decode: {error:?}"),
    };
    let version = match ErrorProvenanceSchemaVersion::try_new(1) {
        Ok(value) => value,
        Err(error) => panic!("the public schema-version fixture must decode: {error:?}"),
    };
    let code = match ErrorProvenanceCode::try_new(code) {
        Ok(value) => value,
        Err(error) => panic!("the public code fixture must decode: {error:?}"),
    };
    ErrorProvenanceDescriptor::new(namespace, version, code)
}

/// Requirement validation: S1-T18.
///
/// Public seam: `kernel_oss::error` re-exports the five provenance values.
/// Logical path: an external consumer attaches, derives, inspects, and clears
/// provenance while retaining the normal `Result<Response, Error>` error type.
/// Observable result: immutable descriptors survive the public transitions and
/// clearing restores an Error with no provenance.
#[test]
fn public_error_provenance_reexports_and_transitions_success() {
    let gateway = descriptor("gateway_failed");
    let application = descriptor("application_failed");
    let error = Error::new(
        Audience::System,
        Kind::GatewayError,
        "Dependency is unavailable.",
    );
    let attached = match error.try_with_attached_provenance(gateway.clone()) {
        Ok(value) => value,
        Err(error) => panic!("public attachment must succeed: {error:?}"),
    };
    let derived = match attached.try_with_derived_provenance(application.clone()) {
        Ok(value) => value,
        Err(error) => panic!("public derivation must succeed: {error:?}"),
    };
    let provenance = match derived.provenance() {
        Some(value) => value,
        None => panic!("public derivation must retain provenance"),
    };
    assert_eq!(provenance.current(), &application);
    assert_eq!(provenance.origin(), Some(&gateway));
    assert!(derived.has_current_provenance(&application));
    let cleared = derived.without_provenance();
    assert!(cleared.provenance().is_none());
    assert_eq!(cleared.audience(), Audience::System);
    assert_eq!(cleared.kind(), Kind::GatewayError);
    assert_eq!(cleared.message(), "Dependency is unavailable.");
}
