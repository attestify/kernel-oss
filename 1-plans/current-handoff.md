# Current Handoff

Date: 2026-06-04

Purpose:

- Record the current stopping point inside `kernel-oss`
- Identify the next recommended implementation step
- Keep follow-up work scoped to files in this repository

## Current State

- Shared kernel seam traits now distinguish request-bearing and no-input execution:
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
- The no-input compatibility retrofits completed so far are:
  - `src/gateway/new_identity/mod.rs`
    - `NewIdentityGW`
    - `AsyncNewIdentityGW`
  - `src/gateway/current_utc_timestamp/mod.rs`
    - `CurrentUTCTimestampGW`
    - `AsyncCurrentUTCTimestampGW`
- The corresponding touched tests are passing:
  - `src/gateway/tests.rs`
  - `src/usecase/tests.rs`
  - `src/gateway/new_identity/tests.rs`
  - `src/gateway/current_utc_timestamp/tests.rs`
- `examples/gateway_usecase_composition.rs` has been updated to use the no-input seam direction.
- `cargo fmt --all` and `cargo test` pass in this repository.

## Where Work Stopped

Work stopped after:

1. Changing true no-input seams away from placeholder request objects and onto dedicated void shared traits.
2. Adding paired async marker seams for the current foundational no-input gateway retrofits.
3. Updating the local planning notes to reflect the new shared seam direction.

No new side-by-side retrofit has started yet for the remaining legacy gateways.

## Immediate Next Step

Retrofit `RetrieveDirectoryPath` next.

Current source:

- `src/gateway/directory_list/mod.rs`

Why this is next:

- It is still a simple compatibility surface: `type RetrieveDirectoryPath = fn(directory_key: &str) -> Result<String, Error>;`
- It is smaller and lower risk than `FileDataGateway`.
- It should let the current compatibility pattern be repeated without introducing the design ambiguity of `Logger`.

Recommended shape:

1. Add a side-by-side replacement module, likely `src/gateway/retrieve_directory_path`.
2. Introduce a bounded request object for the directory key.
3. Add a sync marker seam over `Gateway`.
4. Decide whether the foundational capability should also expose an async marker seam over `AsyncGateway`.
   - The current standards direction says foundational kernel seams should normally provide both sync and async variants when both are reasonable.
5. Keep the existing `RetrieveDirectoryPath` alias temporarily for compatibility.
6. Add sync and async standards-aligned tests before adding deprecation guidance.

## Next Steps After That

1. Retrofit `FileDataGateway` in `src/gateway/file_data_gateway/mod.rs` using the same side-by-side compatibility pattern.
2. Design the `Logger` migration separately before implementation because the current trait is multi-operation and does not fit the one-command seam model.
3. After the remaining gateway retrofits:
   - plan the `ULID` move from `src/ulid` into `src/values`
   - plan the extraction of `src/algorithms`
   - continue broader test cleanup, rustdoc fixes, and clippy cleanup

## Verification Baseline

At this stopping point:

- `cargo fmt --all` passes
- `cargo test` passes
- test count: 307 unit tests
- doctest count: 11 doctests
