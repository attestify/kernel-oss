//! Attaches, derives, inspects, and clears bounded kernel Error provenance.

use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

fn descriptor(code: &str) -> Result<ErrorProvenanceDescriptor, Error> {
    Ok(ErrorProvenanceDescriptor::new(
        ErrorProvenanceNamespace::try_new("attestify.kernel")?,
        ErrorProvenanceSchemaVersion::try_new(1)?,
        ErrorProvenanceCode::try_new(code)?,
    ))
}

fn main() -> Result<(), Error> {
    let gateway = descriptor("gateway_failed")?;
    let application = descriptor("application_failed")?;
    let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    let attached = error.try_with_attached_provenance(gateway.clone())?;
    let derived = attached.try_with_derived_provenance(application.clone())?;

    let provenance = match derived.provenance() {
        Some(value) => value,
        None => {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "Provenance is absent.",
            ));
        }
    };
    assert_eq!(provenance.current(), &application);
    assert_eq!(provenance.origin(), Some(&gateway));
    assert!(derived.has_current_provenance(&application));

    let cleared = derived.without_provenance();
    assert!(cleared.provenance().is_none());
    assert_eq!(cleared.message(), "Dependency is unavailable.");
    Ok(())
}
