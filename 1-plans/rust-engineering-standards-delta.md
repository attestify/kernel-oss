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
   - Tests pass, but broad test cleanup remains.
   - Continue migrating names toward `_success`, `_error`, `_success_async`, and `_error_async`.
   - Prefer explicit setup expectations over plain `.unwrap()` in tests.

8. Documentation
   - `cargo rustdoc --lib -- -D missing_docs` still fails broadly.
   - Broken intra-doc links remain.
   - Add public API docs incrementally after API shape decisions settle.

9. Clippy
   - `cargo clippy --all-targets -- -D warnings` still fails broadly.
   - Address mechanical warnings after the remaining API movement decisions.

## Recommended Active Order

1. Publish the current seam/gateway compatibility release after a clean final verification pass.
2. Plan and implement the `ULID` module move with compatibility re-export.
3. Plan the algorithms/domain split without moving behavior until replacement crates are ready.
4. Add `try_new` / `try_build` compatibility surfaces and migrate internal call sites.
5. Remove recoverable production `.unwrap()` / `.expect(...)` usage.
6. Continue test naming and setup cleanup.
7. Decide public field privacy in a breaking-change plan.
8. Bring rustdoc and clippy to enforceable state.

## Verification Baseline

At the last full release check:

- `cargo fmt --all --check` passed.
- `cargo test` passed.
- `cargo check --examples` passed.
- The release examples passed.
- Test count was 324 unit tests plus 11 doctests.

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
