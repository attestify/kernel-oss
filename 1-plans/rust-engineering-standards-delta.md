# Rust Engineering Standards Delta

Date: 2026-06-17

Scope:

- Track the remaining differences between `kernel-oss` and the Rust engineering standards.
- Keep only current guidance and archived decisions that matter for future implementation.
- Remove dated pickup notes that have already been completed or superseded.

## Current Standards Baseline

The foundational role traits now exist in the kernel:

- `Value` in `src/values/mod.rs`
- `Entity` in `src/entity/mod.rs`
- `ResponseFuture` in `src/response/mod.rs`
- `VoidUseCase`, `UseCase`, `AsyncVoidUseCase`, and `AsyncUseCase` in `src/usecase/mod.rs`
- `VoidGateway`, `Gateway`, `AsyncVoidGateway`, and `AsyncGateway` in `src/gateway/mod.rs`

The current public gateway migration pattern is established:

- Add a standards-aligned side-by-side module.
- Keep the old API temporarily.
- Deprecate the old API with a note pointing to the replacement.
- Add tests proving the replacement before relying on deprecation guidance.
- Do not remove deprecated compatibility APIs without an explicit breaking-change plan.

The current gateway retrofit set is complete for this release:

- `IdentityGateway` -> `gateway::new_identity::NewIdentityGW` / `AsyncNewIdentityGW`
- `UTCTimestampGateway` -> `gateway::current_utc_timestamp::CurrentUTCTimestampGW` / `AsyncCurrentUTCTimestampGW`
- `RetrieveDirectoryPath` -> `gateway::retrieve_directory_path::RetrieveDirectoryPathGW` / `AsyncRetrieveDirectoryPathGW`
- `FileDataGateway` -> `gateway::file_data::FileDataGW` / `AsyncFileDataGW`
- `Logger` -> `gateway::write_log_entry::WriteLogEntryGW` / `AsyncWriteLogEntryGW`

The logger seam decision is closed:

- Logging is one command gateway: `WriteLogEntryGW`.
- `message` is the required log event text.
- `error` is optional structured failure context.
- `level` is request data, not a separate trait method.

The first documentation catalog pass is complete:

- `docs/kernel-catalog.md` is the consumer-facing reuse catalog.
- `docs/documentation-audit.md` tracks documentation maintenance and future additions.
- `README.md` links to the catalog and audit.
- `/docs/**` is packaged with the crate.
- `cargo rustdoc --lib` passes without broken intra-doc link warnings.
- `cargo rustdoc --lib -- -D missing_docs` now passes across the public crate API.

## Active Deltas

1. ULID value-object module boundary
   - `src/ulid` is still a top-level module.
   - `ULID` is a bounded value object and already implements `Value`.
   - Move implementation to `src/values/ulid`.
   - Keep `kernel_oss::ulid::ULID` as a compatibility re-export.
   - Prefer `kernel_oss::values::ulid::ULID` in new docs and examples.

2. Algorithms/domain boundary
   - `src/algorithms` still lives inside the kernel crate.
   - Signature algorithms should move to a future cryptographic domain project.
   - OS home-directory behavior should become a kernel driver trait/seam.
   - The concrete OS implementation should be exported as a kernel driver implementation for that trait.
   - Keep existing APIs until replacement crates and compatibility guidance exist.

3. Construction naming
   - Several fallible constructors still use `new(...) -> Result`.
   - Several fallible builders still use `build(...) -> Result`.
   - Add `try_new` / `try_build` surfaces beside existing APIs, then migrate internal call sites.

4. Builder finalization
   - Some `try_build` methods still borrow `self` instead of consuming the builder.
   - Migrate builders to consuming `self` where compatible with existing usage.

5. Public state surfaces
   - Callable accessors have been added to many bounded public types.
   - Public fields remain where needed for compatibility.
   - Decide field privacy during a breaking-change planning pass, not during the compatibility release.

6. Production unwrap/expect usage
   - Some production `.unwrap()` / `.expect(...)` usage remains.
   - Replace recoverable paths with explicit bounded error handling.

7. Test standard
   - The helper-style conversion pass is complete.
   - Next audit logical-path coverage object by object.
   - Keep `docs/test-audit.md` focused on logical-path coverage.

8. Documentation
   - `docs/kernel-catalog.md` exists and should be kept current as the human-readable reuse catalog.
   - `docs/documentation-audit.md` tracks documentation maintenance and future public API additions.
   - `cargo rustdoc --lib -- -D missing_docs` now passes.
   - Broken intra-doc links were cleaned up in the first catalog pass.
   - Keep public API docs current when adding or changing exported items.

9. Module organization
   - `docs/module-audit.md` tracks the current module-layout deviations.
   - The non-deferred module gaps from the initial audit pass are now addressed.
   - `docs/module-audit.md` now tracks the remaining flat executable backlog across the kernel.
   - `src/values/datetime/start_time/`, `src/values/datetime/utc_timestamp/`, `src/values/directory/name/`, `src/values/directory/directory_list/`, `src/values/file_system/file_name/`, `src/values/nrn/filepath_codec/`, `src/values/specification/name/`, `src/values/specification/procedure/`, `src/values/specification/assurance_procedure/action/`, `src/values/specification/assurance_procedure/activities/`, `src/values/specification/assurance_procedure/activity/`, `src/values/specification/assurance_procedure/artifact/`, `src/values/specification/assurance_procedure/artifacts/`, `src/values/specification/assurance_procedure/procedure/`, `src/values/specification/assurance_report/action/`, `src/values/specification/assurance_report/activities/`, `src/values/specification/assurance_report/activity/`, `src/values/specification/assurance_report/additional_information/`, `src/values/specification/assurance_report/signed_file/`, `src/values/specification/assurance_report/summary/`, `src/values/specification/v1_0_0/assurance_procedure/`, `src/values/specification/v1_0_0/assurance_report/`, `src/values/strings/`, `src/values/text/block/`, `src/values/text/line/`, and `src/values/uri/url/` now use the directory-module shape with colocated tests, alongside the earlier specification value conversions.
   - `src/ulid/mod.rs` still carries roadmap commentary and should move to `src/values/ulid/` when that plan is executed.
   - `src/algorithms/os_home_directory.rs` now has colocated tests and remains part of the flat-module backlog until it is converted into the standard directory shape.

10. Clippy
   - `cargo clippy --all-targets -- -D warnings` now passes cleanly.
   - Keep it that way after future public API and module-layout changes.

11. Test documentation
   - The required test-module doc shape is complete across the current `src/**`
     test tree.
   - Preserve that module/test-doc standard when adding new tests.

## Recommended Active Order

1. Publish the current seam/gateway compatibility release after a clean final verification pass.
2. Plan and implement the `ULID` module move with compatibility re-export.
3. Plan the algorithms/domain split without moving behavior until replacement crates are ready.
4. Add `try_new` / `try_build` compatibility surfaces and migrate internal call sites.
5. Remove recoverable production `.unwrap()` / `.expect(...)` usage.
6. Audit test logical-path coverage and documentation.
7. Decide public field privacy in a breaking-change plan.
8. Keep rustdoc green and clippy enforceable.

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

## Archived Completed Decisions

The following items are closed for the current compatibility release and should not be treated as active pickup work:

- Shared `Value` trait added and implemented for the first bounded value-object set.
- Shared `Entity` trait added in `src/entity/mod.rs`.
- Shared `ResponseFuture` added in `src/response/mod.rs`.
- Shared sync and async use case traits added in `src/usecase/mod.rs`.
- Shared sync and async gateway traits added in `src/gateway/mod.rs`.
- True no-input seams use `VoidUseCase` / `AsyncVoidUseCase` and `VoidGateway` / `AsyncVoidGateway`.
- Request-bearing seams use `UseCase` / `AsyncUseCase` and `Gateway` / `AsyncGateway`.
- Domain-specific `*UC` and `*GW` traits stay as thin marker supertraits over shared roles.
- Request builders stay outside use case and gateway execution.
- No-payload success uses `Response = ()` instead of empty response wrapper objects.
- Current gateway retrofits and deprecations are complete.
- The old dated 2026-05-31 and 2026-06-01 pickup logs are archived by this summary and no longer need to be followed.
