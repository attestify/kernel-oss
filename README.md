# Attestify Kernel OSS

`kernel_oss` is the open-source Rust foundation shared by Attestify libraries
and applications. It provides deterministic value objects, bounded errors,
entity roles, and synchronous and asynchronous use-case and gateway seams.

The crate owns foundational behavior only. It performs no product workflow,
network, database, filesystem, or runtime orchestration.

## Add the crate

Kernel OSS releases are immutable signed Git tags. Use **0.6.0**:

```toml
[dependencies]
kernel_oss = { git = "https://github.com/attestify/kernel-oss.git", tag = "0.6.0" }
```

Version 0.6.0 adds the synchronous operational Logger and explicit no-op. It
retains the typed Usecase Event Handler API from 0.5.0 and bounded Error
provenance from 0.4.0. See the [0.6.0 changelog](CHANGELOG.md#060--2026-10-21) for
compatibility and migration.

## Main capabilities

- `error::Error` with bounded audience, kind, and optional immutable provenance
- reusable value objects and the `values::Value` role
- `entity::Entity` and `response::ResponseFuture`
- synchronous and asynchronous use-case roles
- synchronous and asynchronous gateway roles and kernel gateway markers
- typed synchronous Usecase Event Handler delivery and an explicitly selected no-op
- synchronous operational logging through `logging::Logger` and explicit
  `logging::NoOpLogger` (introduced in 0.6.0)
- `values::text::NfcText`, backed by the shared private Unicode 17 NFC engine
- `values::uri::url::URL`, a dependency-free immutable WHATWG URL value
- specification values including redacted `RepositoryLink`

Use the [Kernel catalog](docs/kernel-catalog.md) before creating a new shared
type or seam.

## Quick start

Parse, inspect, resolve, and immutably update a URL:

```rust
use kernel_oss::values::uri::url::URL;

let base = URL::try_new("https://example.test/a/?one=1")
    .expect("the example URL is valid");
let resolved = base
    .try_resolve("../b")
    .expect("the relative reference resolves");
let updated = resolved
    .mutator()
    .query(Some("two=2"))
    .try_mutate()
    .expect("the component replacement is valid");

assert_eq!(updated.scheme(), "https");
assert_eq!(updated.host(), Some("example.test"));
assert_eq!(updated.query_pairs(), &[("two".to_owned(), "2".to_owned())]);
```

Hold text in canonical NFC form:

```rust
use kernel_oss::values::Value;
use kernel_oss::values::text::NfcText;

let text = NfcText::new("e\u{301}");
assert_eq!(text.value(), "é");
```

Construct an Error directly with its three small, infallible, fixed inputs;
there is intentionally no `ErrorBuilder`. Initial construction accepts no
provenance. A semantic owner first creates an immutable fallible definition set
and can then attach, derive, inspect, and clear provenance through immutable
Error reconstruction while retaining the normal `Result<Response, Error>`
failure channel:

```rust
use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};

struct LocalProvenanceDefinitions {
    gateway_failure: ErrorProvenanceDescriptor,
    application_failure: ErrorProvenanceDescriptor,
}

impl LocalProvenanceDefinitions {
    fn try_new() -> Result<Self, Error> {
        let namespace = ErrorProvenanceNamespace::try_new("example.owner")?;
        let schema_version = ErrorProvenanceSchemaVersion::try_new(1)?;
        Ok(Self {
            gateway_failure: ErrorProvenanceDescriptor::new(
                namespace.clone(), schema_version,
                ErrorProvenanceCode::try_new("gateway_failure")?,
            ),
            application_failure: ErrorProvenanceDescriptor::new(
                namespace, schema_version,
                ErrorProvenanceCode::try_new("application_failure")?,
            ),
        })
    }

    fn gateway_failure(&self) -> &ErrorProvenanceDescriptor { &self.gateway_failure }
    fn application_failure(&self) -> &ErrorProvenanceDescriptor { &self.application_failure }
}

fn main() -> Result<(), Error> {
    let definitions = LocalProvenanceDefinitions::try_new()?;
    let error = Error::for_system(Kind::GatewayError, "Dependency is unavailable.");
    assert!(error.provenance().is_none());
    let attached = error.try_with_attached_provenance(definitions.gateway_failure().clone())?;
    let derived = attached
        .try_with_derived_provenance(definitions.application_failure().clone())?;
    let provenance = match derived.provenance() {
        Some(value) => value,
        None => return Err(Error::for_system(Kind::ProcessingFailure, "Provenance is absent.")),
    };
    assert_eq!(provenance.current(), definitions.application_failure());
    assert_eq!(provenance.origin(), Some(definitions.gateway_failure()));
    assert!(derived.without_provenance().provenance().is_none());
    Ok(())
}
```

## Shared seams

Use cases:

- `usecase::VoidUseCase`
- `usecase::UseCase`
- `usecase::AsyncVoidUseCase`
- `usecase::AsyncUseCase`

Gateways:

- `gateway::VoidGateway`
- `gateway::Gateway`
- `gateway::AsyncVoidGateway`
- `gateway::AsyncGateway`

Gateway markers include new identity, current UTC timestamp, directory-path
retrieval and file-data retrieval. The log-entry writing Gateways remain
compatibility APIs. New operational logging uses `logging::Logger` with
`info`, `warn` and `error`; `logging::NoOpLogger` explicitly discards messages.
These operational logging APIs are introduced in 0.6.0. See the
[logging contract](docs/logging/README.md) for construction, safety
and migration, and the [Kernel catalog](docs/kernel-catalog.md) for compatibility
paths.

Typed delivery uses `usecase_event_handler::UsecaseEventHandler` and
`usecase_event_handler::NoOpUsecaseEventHandler`. Domain markers fix the
finalized input and own permitted no-op bindings. Application selects and
injects the Handler explicitly. No-op success does not establish output;
required delivery cannot use a no-op. See the
[Handler contract](docs/kernel-catalog.md#usecase-event-handler) for exact bounds
and construction behavior.

## Security and data handling

`URL::value()` returns the complete canonical URL and can contain credentials,
query data, and a fragment. Treat it as sensitive. `URL` has redacted `Debug`
and intentionally has no `Display`. `RepositoryLink` also redacts `Display` and
`Debug`.

`NfcText` and the URL engine expose no public normalizer, parser engine, IDNA
engine, generated table, or mutable internal state. Public Rust APIs are source
contracts; the crate makes no stable memory-layout, FFI, or binary-ABI promise.

Error provenance is bounded semantic classification, not a metadata channel.
It contains at most one current and one origin descriptor; it cannot carry
transport values, native diagnostics, source chains, arbitrary metadata,
callbacks, or product-specific constants. `Error` retains complete equality but
intentionally does not implement `Hash`.

`Error::new`, `Error::for_user`, and `Error::for_system` are the deliberate
direct-construction exception for this small, infallible Kernel value. There is
no `ErrorBuilder`, builder setter, generic field setter, or initial-construction
provenance input. Provenance starts absent and changes only through immutable
attach, derive, and clear reconstruction transitions.

## Architecture and reference documentation

- [Documentation index](docs/README.md): consumer guides and the library
  documentation convention
- [Kernel catalog](docs/kernel-catalog.md): reusable types, seams, and
  compatibility paths
- [Operational logging](docs/logging/README.md): synchronous Logger, explicit no-op,
  concrete implementation obligations and legacy migration
- [URL value and architecture](docs/values/uri/url/README.md): public contract, resolution,
  mutation, security, and private engine composition
- [NFC text](docs/values/text/README.md): canonical text behavior, shared normalization,
  and Unicode custody
- [Test architecture](docs/testing.md): unit, private-conformance, integration,
  fixture, and generator boundaries
- [README standard](docs/readme-standard.md): required structure and content
  rules for this README
- [Changelog](CHANGELOG.md): release history, compatibility notes and
  migration guidance, including the 0.4.0 Error provenance transition
- Rustdoc: generated API reference from the public source documentation

## Examples

Run checked examples from the repository root:

```text
cargo run --example value_object_and_entity
cargo run --example gateway_usecase_composition
cargo run --example async_gateway_usecase_composition
cargo run --example unit_success_payload
cargo run --example error_provenance
```

## Verification

`make ci` is a fast contributor check (format check, lint and tests), not the
full release baseline. It does not edit source to repair formatting. Use
`make fmt` when you intend to apply formatting. For locked offline contributor
checks, run `make ci CARGO_FLAGS="--locked --offline"`.

Run the full baseline below before a release, plus the area-specific checks
described in [Test architecture](docs/testing.md):

```text
cargo fmt --all --check
cargo clippy --all-targets --locked --offline -- -D warnings
cargo test --all-targets --locked --offline
cargo test --doc --locked --offline
cargo check --examples --locked --offline
cargo build --release --locked --offline
```

The repository also contains deterministic offline generators and conformance
fixtures for NFC, URL, and IDNA. Their source and license custody is documented
in `tests/data/`, the owning `tools/**/data/` directories, and
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Versioning and license

Pre-1.0 minor releases may contain breaking public Rust API changes. Pin an
immutable signed release tag and rebuild consumers when upgrading.

Kernel OSS is distributed under the repository [license](LICENSE). Retained
third-party notices and license texts are listed in
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
