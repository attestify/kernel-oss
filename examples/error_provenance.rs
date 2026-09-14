//! Attaches, derives, inspects, and clears bounded Kernel Error provenance.
//!
//! A semantic owner constructs its validated immutable definition set through
//! `try_new`. The Kernel supplies bounded descriptor values and Error
//! reconstruction transitions, but does not own this example's meanings.

use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

struct ExampleProvenanceDefinitions {
    gateway_failure: ErrorProvenanceDescriptor,
    application_failure: ErrorProvenanceDescriptor,
}

impl ExampleProvenanceDefinitions {
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
                namespace,
                schema_version,
                ErrorProvenanceCode::try_new("application_failure")?,
            ),
        })
    }

    fn gateway_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.gateway_failure
    }

    fn application_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.application_failure
    }
}

fn main() -> Result<(), Error> {
    let definitions = ExampleProvenanceDefinitions::try_new()?;
    let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    let attached = error.try_with_attached_provenance(definitions.gateway_failure().clone())?;
    let derived =
        attached.try_with_derived_provenance(definitions.application_failure().clone())?;

    let provenance = match derived.provenance() {
        Some(value) => value,
        None => {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "Provenance is absent.",
            ));
        }
    };
    assert_eq!(provenance.current(), definitions.application_failure());
    assert_eq!(provenance.origin(), Some(definitions.gateway_failure()));
    assert!(derived.has_current_provenance(definitions.application_failure()));

    let cleared = derived.without_provenance();
    assert!(cleared.provenance().is_none());
    assert_eq!(cleared.message(), "Dependency is unavailable.");
    Ok(())
}
