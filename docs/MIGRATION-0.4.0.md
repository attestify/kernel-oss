# Kernel OSS 0.4.0 migration guide

## Scope

Kernel OSS 0.4.0 adds bounded immutable provenance to `error::Error`. The
normal failure shape remains `Result<Response, Error>`.

## Construct errors through supported constructors

Create errors with `Error::new`, `Error::for_user`, or `Error::for_system`.
The provenance field is private so direct `Error { ... }` construction is not
supported by the 0.4.0 API. Existing outer fields remain readable for source
compatibility while consumers transition to the accessors. These three direct
constructors are the deliberate small, infallible Error exception: there is no
`ErrorBuilder`, builder setter, generic field setter, or provenance input at
initial construction. Every new Error starts without provenance.

## Attach, derive, inspect, and clear

An owner constructs its validated immutable definition set through a fallible
`try_new()` path, then its named accessors provide descriptors for immutable
Error transitions. Kernel does not define product descriptor meanings:

```rust
use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

struct LocalProvenanceDefinitions {
    dependency_failure: ErrorProvenanceDescriptor,
}

impl LocalProvenanceDefinitions {
    fn try_new() -> Result<Self, Error> {
        Ok(Self {
            dependency_failure: ErrorProvenanceDescriptor::new(
                ErrorProvenanceNamespace::try_new("example.owner")?,
                ErrorProvenanceSchemaVersion::try_new(1)?,
                ErrorProvenanceCode::try_new("dependency_failure")?,
            ),
        })
    }

    fn dependency_failure(&self) -> &ErrorProvenanceDescriptor {
        &self.dependency_failure
    }
}

fn main() -> Result<(), Error> {
    let definitions = LocalProvenanceDefinitions::try_new()?;
    let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    let attached = error.try_with_attached_provenance(definitions.dependency_failure().clone())?;
    assert!(attached.provenance().is_some());
    Ok(())
}
```

`try_with_derived_provenance` is permitted only for a current-only descriptor;
the previous current descriptor becomes the one origin. Both fallible
transitions return exact bounded Kernel errors and leave the original borrowed
`Error` unchanged. `without_provenance` reconstructs an Error with the same
outer state and no provenance. Attach, derive, and clear are immutable
reconstruction transitions, not builder finalization or mutation.

## Equality and hashing

`Error` equality includes audience, kind, message, current provenance, and
origin provenance. `Error` no longer implements `Hash`; replace `HashMap` or
`HashSet` error keys with an explicit stable key that matches the caller's
actual semantics.

## Consumer boundary

This document describes the Kernel source transition only. Downstream consumer
migration and compatibility qualification are separately owned; this change
does not assert their completion.
