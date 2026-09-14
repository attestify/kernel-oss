# Attestify Kernel OSS

`kernel_oss` is the open-source Rust foundation shared by Attestify libraries
and applications. It provides deterministic value objects, bounded errors,
entity roles, and synchronous and asynchronous use-case and gateway seams.

The crate owns foundational behavior only. It performs no product workflow,
network, database, filesystem, or runtime orchestration.

## Add the crate

Kernel OSS releases are immutable signed Git tags:

```toml
[dependencies]
kernel_oss = { git = "https://github.com/attestify/kernel-oss.git", tag = "0.3.0" }
```

The 0.4.0 API described in this source tree is not a published tag. Consumers
should use an issued immutable tag until its separate distribution decision.

## Main capabilities

- `error::Error` with bounded audience, kind, and optional immutable provenance
- reusable value objects and the `values::Value` role
- `entity::Entity` and `response::ResponseFuture`
- synchronous and asynchronous use-case roles
- synchronous and asynchronous gateway roles and kernel gateway markers
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

Attach and later derive bounded semantic provenance while retaining the normal
`Result<Response, Error>` failure channel:

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
    .try_with_attached_provenance(descriptor.clone())
    .expect("provenance is absent");
let derived_descriptor = ErrorProvenanceDescriptor::new(
    ErrorProvenanceNamespace::try_new("attestify.application").expect("valid namespace"),
    ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero version"),
    ErrorProvenanceCode::try_new("request_failed").expect("valid code"),
);
let derived = attached
    .try_with_derived_provenance(derived_descriptor.clone())
    .expect("current-only provenance is derivable");
let provenance = derived.provenance().expect("derived provenance is present");
assert_eq!(provenance.current(), &derived_descriptor);
assert_eq!(provenance.origin(), Some(&descriptor));
assert!(derived.without_provenance().provenance().is_none());
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

Standards-aligned gateway markers include new identity, current UTC timestamp,
directory-path retrieval, file-data retrieval, and log-entry writing. Legacy
gateway compatibility modules remain available for existing consumers; prefer
the replacements listed in the [Kernel catalog](docs/kernel-catalog.md).

## Security and data handling

`URL::value()` returns the complete canonical URL and can contain credentials,
query data, and a fragment. Treat it as sensitive. `URL` has redacted `Debug`
and intentionally has no `Display`. `RepositoryLink` also redacts `Display` and
`Debug`.

`NfcText` and the URL engine expose no public normalizer, parser engine, IDNA
engine, generated table, or mutable internal state. Public Rust APIs are source
contracts; the crate makes no stable memory-layout, FFI, or binary-ABI promise.

Error provenance is bounded semantic classification, not a metadata channel.
It contains at most one current and one origin descriptor; it cannot carry HTTP
values, native diagnostics, source chains, arbitrary metadata, callbacks, or
product-specific constants. `Error` retains complete equality but intentionally
does not implement `Hash`.

## Architecture and reference documentation

- [Kernel catalog](docs/kernel-catalog.md): reusable types, seams, and
  compatibility paths
- [URL value and architecture](docs/url.md): public contract, resolution,
  mutation, security, and private engine composition
- [NFC text](docs/nfc-text.md): canonical text behavior, shared normalization,
  and Unicode custody
- [Test architecture](docs/testing.md): unit, private-conformance, integration,
  fixture, and generator boundaries
- [README standard](docs/readme-standard.md): required structure and content
  rules for this README
- [0.4.0 migration guide](docs/MIGRATION-0.4.0.md): bounded Error provenance
  source transition and consumer inventory guidance
- [0.4.0 semver report](docs/SEMVER-0.4.0.md): public API evolution summary
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
