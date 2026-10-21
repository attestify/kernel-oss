# Kernel OSS Repository Guidance

## Scope

Work only from this repository's current source, tests, and durable
documentation. Do not treat historical implementation plans, external evidence
packets, or adjacent repositories as authority for Kernel OSS.

## Read first

1. `README.md`
2. `docs/kernel-catalog.md`
3. `docs/readme-standard.md`
4. `Cargo.toml`

For URL or NFC changes, also read:

- `docs/values/uri/url/README.md` and `docs/values/uri/url/design.md`
- `docs/values/text/README.md` and `docs/values/text/design.md`
- `THIRD_PARTY_NOTICES.md`
- the behavioral-fixture custody under `tests/data/` and generation inputs
  under the owning `tools/**/data/` directory

For test-layout, fixture-location, or conformance-suite changes, also read
`docs/testing.md` and run its discovery guard before and after the change.

## Ownership boundaries

- Kernel OSS owns shared foundational behavior and must remain independently
  buildable from its declared open-source dependency graph.
- The crate owns one private NFC engine. `NfcText` and private URL/IDNA code
  share it; do not add another normalizer.
- The URL facade, parser, serializer, resolver, host processing, IDNA,
  percent/form processing, and generated tables remain dependency-free and
  private except for the documented public URL API.
- Preserve redacted diagnostics and explicit sensitive-value access.
- Public Rust APIs are source contracts, not stable memory-layout, FFI, or
  binary-ABI contracts.

## Compatibility and design

- Use the shared roles in `src/usecase/mod.rs`, `src/gateway/mod.rs`,
  `src/entity/mod.rs`, `src/response/mod.rs`, and `src/values/mod.rs`.
- Prefer existing cataloged types and seams over duplicate abstractions.
- Keep domain-specific gateway and use-case seams as thin roles over the shared
  traits.
- Do not remove a compatibility API without an explicitly authorized breaking
  change and a consumer migration path.
- Keep generated source reproducible from its committed generator and pinned
  inputs.
- Put durable consumer or architecture guidance under `docs/`; do not create
  implementation handoff or evidence directories in the product repository.

## Verification

Run the checks appropriate to the change, with the full baseline before a
release:

```text
cargo fmt --all --check
cargo clippy --all-targets --locked --offline -- -D warnings
cargo test --all-targets --locked --offline
cargo test --doc --locked --offline
cargo check --examples --locked --offline
cargo build --release --locked --offline
```

URL changes must also keep the committed generators in check mode and the
pinned conformance suites green. NFC changes must reproduce the private table
and pass the official Unicode normalization suite.

## Documentation maintenance

Update the relevant formal document whenever a public path, ownership
boundary, security rule, source-data custody rule, or generator workflow
changes. Keep `README.md` conformant with `docs/readme-standard.md`.
