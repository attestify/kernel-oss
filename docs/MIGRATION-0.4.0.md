# Kernel OSS 0.4.0 migration guide

## Scope

Kernel OSS 0.4.0 adds bounded immutable provenance to `error::Error`. The
normal failure shape remains `Result<Response, Error>`.

## Construct errors through supported constructors

Create errors with `Error::new`, `Error::for_user`, or `Error::for_system`.
The provenance field is private so direct `Error { ... }` construction is not
supported by the 0.4.0 API. Existing outer fields remain readable for source
compatibility while consumers transition to the accessors.

## Attach, derive, inspect, and clear

Construct a descriptor only from bounded namespace, nonzero schema-version,
and code values. Then attach it to an error with no provenance:

```rust
use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

let descriptor = ErrorProvenanceDescriptor::new(
    ErrorProvenanceNamespace::try_new("attestify.kernel").expect("valid namespace"),
    ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ErrorProvenanceCode::try_new("dependency_failed").expect("valid code"),
);
let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
let attached = error
    .try_with_attached_provenance(descriptor)
    .expect("provenance is absent");
assert!(attached.provenance().is_some());
```

`try_with_derived_provenance` is permitted only for a current-only descriptor;
the previous current descriptor becomes the one origin. Both fallible
transitions return exact bounded Kernel errors and leave the original borrowed
`Error` unchanged. `without_provenance` reconstructs an Error with the same
outer state and no provenance.

## Equality and hashing

`Error` equality includes audience, kind, message, current provenance, and
origin provenance. `Error` no longer implements `Hash`; replace `HashMap` or
`HashSet` error keys with an explicit stable key that matches the caller's
actual semantics.

## Consumer boundary

This document describes the Kernel source transition only. Downstream consumer
migration and compatibility qualification are separately owned; this change
does not assert their completion.
