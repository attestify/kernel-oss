# Current Handoff

Date: 2026-06-17

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
- The request-bearing compatibility retrofits completed so far are:
  - `src/gateway/retrieve_directory_path/mod.rs`
    - `RetrieveDirectoryPathRequest`
    - `RetrieveDirectoryPathGW`
    - `AsyncRetrieveDirectoryPathGW`
    - `RetrieveDirectoryPathFnGateway`
  - `src/gateway/file_data/mod.rs`
    - `FileDataRequest`
    - `FileDataGW`
    - `AsyncFileDataGW`
    - `FileDataFnGateway`
  - `src/gateway/write_log_entry/mod.rs`
    - `LogLevel`
    - `WriteLogEntryRequest`
    - `WriteLogEntryGW`
    - `AsyncWriteLogEntryGW`
    - `WriteLogEntryFnGateway`
- The corresponding touched tests are passing:
  - `src/gateway/tests.rs`
  - `src/usecase/tests.rs`
  - `src/gateway/new_identity/tests.rs`
  - `src/gateway/current_utc_timestamp/tests.rs`
  - `src/gateway/retrieve_directory_path/tests.rs`
  - `src/gateway/file_data/tests.rs`
  - `src/gateway/write_log_entry/tests.rs`
- `examples/gateway_usecase_composition.rs` has been updated to use the no-input seam direction.
- `cargo fmt --all` and `cargo test` pass in this repository.

## Where Work Stopped

Work stopped after:

1. Changing true no-input seams away from placeholder request objects and onto dedicated void shared traits.
2. Adding paired async marker seams for the current foundational no-input gateway retrofits.
3. Adding side-by-side request-bearing gateway retrofits for `RetrieveDirectoryPath` and `FileDataGateway`.
4. Adding a side-by-side write-log-entry gateway retrofit for `Logger`.
5. Deprecating the old gateway aliases and logger trait with migration notes pointing to the replacement modules.
6. Updating release-facing package metadata and README guidance.

The `Logger` migration decision is now recorded in code and docs: logging is one command gateway, `WriteLogEntryGW`, with `LogLevel` as request data. `message` is the primary log event text, while `error` is optional structured failure context.

## Immediate Next Step

Plan the `ULID` move from `src/ulid` into `src/values`.

## Next Steps After That

1. Plan the extraction of `src/algorithms`.
2. Continue broader test cleanup, rustdoc fixes, and clippy cleanup.

## Verification Baseline

At this stopping point:

- `cargo fmt --all` passes
- `cargo test` passes
- test count: 324 unit tests
- doctest count: 11 doctests
