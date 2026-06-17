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
- Test count was 324 unit tests plus 11 doctests.

Before publishing `0.2.6`, rerun:

```text
cargo fmt --all --check
cargo test
cargo check --examples
cargo publish --dry-run --allow-dirty
```

## Known Non-Blocking Release Gaps

These are still real standards deltas, but they do not block the current seam/gateway compatibility release unless the team decides to raise the release bar:

- `cargo clippy --all-targets -- -D warnings` still fails broadly.
- `cargo clippy --all-targets -- -D warnings` now passes cleanly.
- `cargo rustdoc --lib -- -D missing_docs` now passes.
- Some bounded public types still expose public fields for compatibility.
- Several fallible constructors and builders still need `try_new` / `try_build` migration work.
- Some existing builders still borrow `self` during finalization.
- Production `.unwrap()` / `.expect(...)` cleanup is not complete.
- Test naming and test `.unwrap()` cleanup remain broad backlog work.

## Immediate Next Recommended Task

Plan and implement the `ULID` module move:

- Move the implementation from `src/ulid` to `src/values/ulid`.
- Keep `kernel_oss::ulid::ULID` available as a compatibility re-export.
- Prefer `kernel_oss::values::ulid::ULID` in new docs and examples.
- Decide whether the old top-level module should be deprecated immediately or kept without deprecation until the next breaking-change plan is drafted.

## After That

1. Plan the algorithms/domain split.
   - Signature algorithms should move to a future cryptographic domain project.
   - OS home-directory behavior should become a kernel driver trait/seam.
   - The concrete OS implementation should be exported as a kernel driver implementation for that trait.
   - Keep current `src/algorithms` APIs until replacements and compatibility guidance exist.
2. Add `try_new` / `try_build` compatibility surfaces and migrate internal call sites.
3. Remove recoverable production `.unwrap()` / `.expect(...)` usage.
4. Continue test cleanup and rustdoc cleanup.

## Archived Completed Work

The old dated pickup logs for the initial shared-trait work and gateway retrofit sequence have been collapsed into this file and `1-plans/rust-engineering-standards-delta.md`. They no longer need to be followed as active instructions.
