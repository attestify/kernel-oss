# Kernel OSS changelog

Release changes, compatibility notes and migration guidance are collected here.
Version headings identify crate releases; unreleased work is explicitly marked.
This consolidates the existing release documents, not a complete history of all
earlier releases.

## 0.6.0 — 2026-10-21

Release tag: `0.6.0`. Tags use plain version numbers, without a prefix.
This is the crate version; Logger design document revision 0.1.0 is a separate
identifier.

### Added

- `kernel_oss::logging::Logger`: `Send + Sync`, with required synchronous
  `info(&self, &str)`, `warn(&self, &str)` and `error(&self, &str)` methods.
  All return unit; a normal return is not a delivery acknowledgment.
- `kernel_oss::logging::NoOpLogger`: explicit infallible `new()` and `Default`,
  with no allocation, I/O, retained input, invocation state or diagnostics.
- Self-contained [consumer documentation](docs/logging/README.md), [design](docs/logging/design.md)
  and [source layout](docs/logging/source-layout.md), including concrete logger
  confidentiality and failure-diagnostic obligations.
- Ordinary-interface tests, trait-object/concurrency tests and compiler fixtures
  verifying the actual public library, including silent no-op execution.

### Changed

- Legacy `gateway::logger::Logger` guidance now points to the operational Logger
  and explicitly requires signature adaptation. Its existing deprecation remains.
- Log-entry writing Gateways are documented as compatibility request/Result APIs.
- Documentation and source comments describe the OSS contracts directly within
  this repository. Consumer guides are organized by public API area with README
  landing pages and linked design references; `docs/README.md` provides the index
  and local authoring convention.
- Corrected catalog imports, constructors and FilePath validation guidance.
- Added non-mutating `make fmt-check`; `make ci` uses it and remains a fast
  contributor check, separate from the documented full release baseline.
- Added complete URL generator verification instructions and corrected the NFC
  fixture checksum link.

### Compatibility and migration

The API addition requires no new runtime or development dependency. Existing
Logger methods, logging Gateway Request builder/validation, sync/async adapters,
Error/provenance, four Usecase/four Gateway bases and typed Handler API remain.
No existing executable API is removed or newly deprecated. Public contracts are
Rust source APIs, not stable memory-layout, FFI or binary-ABI promises.

Existing consumers may retain legacy logging. To opt into the new operational
interface, implement the three required methods and import
`kernel_oss::logging::Logger` for method lookup. Use `warn` rather than legacy
`warning`; there is no `debug`, Error/context argument, Result, initialization
hook or async method. Select safe messages explicitly. No automatic adapter is
supplied. Choose NoOpLogger explicitly when absence of output is intended.

`WriteLogEntryGW` and `AsyncWriteLogEntryGW` retain their request/Result contracts;
changing to the operational Logger is a caller design decision, not an automatic
migration. See [Compatibility and migration](docs/logging/README.md#compatibility-and-migration).
Pin tag `0.6.0`, rebuild, and verify consuming applications.
This changelog does not claim downstream adoption or qualification.

### Boundaries

No production sink, global installation, queue, worker, preparation hook, health
API, async Logger or new Error kind is added. Concrete implementations own safe
preparation, synchronization, bounded failure handling and diagnostic routing.
No-op tests do not qualify real sink behavior.

## 0.4.0

### Added

- Immutable bounded error-provenance values:
  `ErrorProvenanceNamespace`, `ErrorProvenanceCode`,
  `ErrorProvenanceSchemaVersion`, `ErrorProvenanceDescriptor`, and
  `ErrorProvenance`.
- `Error` reconstruction methods for attaching one current descriptor,
  deriving one new current descriptor with one origin, inspecting provenance,
  and clearing provenance.
- Public Rustdoc, executable doctests, and an `error_provenance` example for
  the bounded provenance path.

### Changed

- `Error` equality now explicitly includes its optional current and origin
  provenance state.
- `Error` intentionally no longer implements `Hash`; a hash identity that
  omitted provenance would be semantically incomplete.
- `Error` retains its normal `Result<Response, Error>` failure carriage. No
  response wrapper or alternate failure channel is introduced.

### Excluded

This source change adds no registries, arbitrary metadata, dynamic payloads,
source chains, HTTP values, vendor diagnostics, SQLSTATE values, or
product-specific descriptors.

### Migration guide

#### Scope

Kernel OSS 0.4.0 adds bounded immutable provenance to `error::Error`. The
normal failure shape remains `Result<Response, Error>`.

#### Construct errors through supported constructors

Create errors with `Error::new`, `Error::for_user`, or `Error::for_system`.
The provenance field is private so direct `Error { ... }` construction is not
supported by the 0.4.0 API. Existing outer fields remain readable for source
compatibility while consumers transition to the accessors. These three direct
constructors are the deliberate small, infallible Error exception: there is no
`ErrorBuilder`, builder setter, generic field setter, or provenance input at
initial construction. Every new Error starts without provenance.

#### Attach, derive, inspect, and clear

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

#### Equality and hashing

`Error` equality includes audience, kind, message, current provenance, and
origin provenance. `Error` no longer implements `Hash`; replace `HashMap` or
`HashSet` error keys with an explicit stable key that matches the caller's
actual semantics.

#### Consumer boundary

This document describes the Kernel source transition only. Downstream consumer
migration and compatibility qualification are separately owned; this change
does not assert their completion.

### API compatibility

#### Public API change

`Error` gains private optional provenance state and therefore direct external
struct-literal construction is source-incompatible. The supported constructors
and existing audience, kind, message, display, and accessor behavior remain
available. `Error::new`, `Error::for_user`, and `Error::for_system` remain the
deliberate direct-construction exception for this small, infallible value; no
`ErrorBuilder`, builder setter, generic field setter, or initial provenance
input is introduced.

The release adds five public bounded provenance types and the following Error
methods:

- `try_with_attached_provenance`
- `try_with_derived_provenance`
- `without_provenance`
- `provenance`
- `has_current_provenance`

Every direct constructor starts with absent provenance. Attach, derive, and
clear return reconstructed immutable Errors rather than mutating an existing
Error or finalizing a builder.

#### Intentional trait change

`Error` retains `Clone`, `Debug`, `PartialEq`, and `Eq`. Equality includes all
provenance state. `Hash` is intentionally removed so no caller can use a hash
identity that omits current or origin provenance.

#### Compatibility boundary

This is a pre-1.0 minor-version source evolution. Consumers that construct
`Error` with a struct literal or use it as a hash key must migrate to the
supported constructors/accessors or an explicit caller-owned key. This report
does not claim downstream consumer compatibility or publication status.
