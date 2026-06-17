# Current Handoff

Date: 2026-06-17

Purpose:

- Record the current release state inside `kernel-oss`.
- Keep the next work focused on remaining release decisions.
- Remove stale pickup notes that no longer guide current implementation.

## Current Release State

The shared kernel role traits are in place:

- `src/values/mod.rs`
  - `Value`
- `src/entity/mod.rs`
  - `Entity`
- `src/response/mod.rs`
  - `ResponseFuture`
- `src/usecase/mod.rs`
  - `VoidUseCase`
  - `UseCase`
  - `AsyncVoidUseCase`
  - `AsyncUseCase`
- `src/gateway/mod.rs`
  - `VoidGateway`
  - `Gateway`
  - `AsyncVoidGateway`
  - `AsyncGateway`

The current gateway compatibility retrofits are complete:

- `IdentityGateway` is deprecated in favor of `gateway::new_identity::NewIdentityGW` and `AsyncNewIdentityGW`.
- `UTCTimestampGateway` is deprecated in favor of `gateway::current_utc_timestamp::CurrentUTCTimestampGW` and `AsyncCurrentUTCTimestampGW`.
- `RetrieveDirectoryPath` is deprecated in favor of `gateway::retrieve_directory_path::RetrieveDirectoryPathGW` and `AsyncRetrieveDirectoryPathGW`.
- `FileDataGateway` is deprecated in favor of `gateway::file_data::FileDataGW` and `AsyncFileDataGW`.
- `Logger` is deprecated in favor of `gateway::write_log_entry::WriteLogEntryGW` and `AsyncWriteLogEntryGW`.

The logger design decision is closed for this release:

- Logging is modeled as one command gateway: `WriteLogEntryGW`.
- `WriteLogEntryRequest::message` is the primary log event text.
- `WriteLogEntryRequest::error` is optional structured failure context.
- `LogLevel` is request data, not a separate operation.

The first documentation catalog pass is complete:

- `docs/kernel-catalog.md` is the human-readable reuse catalog for consumers.
- `docs/documentation-audit.md` tracks remaining rustdoc cleanup.
- `README.md` links to both documentation files.
- `/docs/**` is included in the crate package.
- `cargo rustdoc --lib` passes without broken intra-doc link warnings.
- `cargo rustdoc --lib -- -D missing_docs` now passes for the public crate API.
- `docs/test-audit.md` records the current test inventory and the remaining
  logical-path audit pass.
- `docs/module-audit.md` records the current module-layout deviations and the
  remaining module-organization cleanup.
- `src/values/directory/directory_list/`, `src/values/file_system/file_name/`, `src/values/nrn/filepath_codec/`, `src/values/specification/api_version/`, `src/values/specification/description/`, `src/values/specification/file_path/`, `src/values/specification/kind/`, `src/values/specification/metadata/`, `src/values/specification/name/`, `src/values/specification/outcome/`, `src/values/specification/procedure/`, `src/values/specification/repository_link/`, `src/values/specification/short_description/`, `src/values/specification/subject/`, `src/values/specification/subject_id/`, `src/values/strings/`, `src/values/text/block/`, `src/values/text/line/`, and `src/values/uri/url/` now use the directory-module shape with colocated tests.
- `src/values/specification/assurance_procedure/action/`, `src/values/specification/assurance_procedure/activities/`, `src/values/specification/assurance_procedure/activity/`, `src/values/specification/assurance_procedure/artifact/`, `src/values/specification/assurance_procedure/artifacts/`, `src/values/specification/assurance_procedure/procedure/`, `src/values/specification/assurance_report/action/`, `src/values/specification/assurance_report/activities/`, `src/values/specification/assurance_report/activity/`, `src/values/specification/assurance_report/additional_information/`, `src/values/specification/assurance_report/signed_file/`, `src/values/specification/assurance_report/summary/`, `src/values/specification/v1_0_0/assurance_procedure/`, and `src/values/specification/v1_0_0/assurance_report/` now use the directory-module shape with colocated tests.

Compatibility rule:

- Keep deprecated public APIs until a breaking-change plan explicitly schedules removal.
- Add standards-aligned APIs side by side before removing any existing public surface.

## Verification Baseline

At the last full release check:

- `cargo fmt --all --check` passed.
- `cargo test` passed.
- `cargo check --examples` passed.
- `cargo rustdoc --lib` passed.
- The release examples passed.
- Test count was 355 unit tests plus 11 doctests.

Before publishing `0.2.6`, rerun:

```text
cargo fmt --all --check
cargo test
cargo check --examples
cargo publish --dry-run --allow-dirty
```

## Known Non-Blocking Release Gaps

These are still real standards deltas, but they do not block the current seam/gateway compatibility release unless the team decides to raise the release bar:

- `cargo clippy --all-targets -- -D warnings` now passes cleanly.
- `cargo rustdoc --lib -- -D missing_docs` now passes.
- Some bounded public types still expose public fields for compatibility.
- Several fallible constructors and builders still need `try_new` / `try_build` migration work.
- Some existing builders still borrow `self` during finalization.
- Production `.unwrap()` / `.expect(...)` cleanup is not complete.
- `docs/test-audit.md` now tracks the next logical-path coverage audit.
- `docs/module-audit.md` now tracks the current module-organization audit.

## Immediate Next Recommended Task

The non-deferred module backlog is now closed for this release pass.

- keep `src/values/copy_value.rs` flat unless it grows executable behavior
- defer the ULID move by request
- the algorithms/domain split remains the next planned architecture change

## After That

1. Plan the algorithms/domain split.
   - Signature algorithms should move to a future cryptographic domain project.
   - OS home-directory behavior should become a kernel driver trait/seam.
   - The concrete OS implementation should be exported as a kernel driver implementation for that trait.
   - Keep current `src/algorithms` APIs until replacements and compatibility guidance exist.
2. Add `try_new` / `try_build` compatibility surfaces and migrate internal call sites.
3. Remove recoverable production `.unwrap()` / `.expect(...)` usage.
4. Continue rustdoc cleanup and any remaining test-path coverage cleanup.

## Archived Completed Work

The old dated pickup logs for the initial shared-trait work, gateway retrofit
sequence, and kernel test helper cleanup have been collapsed into this file and
`1-plans/rust-engineering-standards-delta.md`. They no longer need to be
followed as active instructions.
