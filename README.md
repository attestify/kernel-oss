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

## Main capabilities

- `error::Error` with bounded audience and kind classifications
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
- Rustdoc: generated API reference from the public source documentation

## Examples

Run checked examples from the repository root:

```text
cargo run --example value_object_and_entity
cargo run --example gateway_usecase_composition
cargo run --example async_gateway_usecase_composition
cargo run --example unit_success_payload
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
