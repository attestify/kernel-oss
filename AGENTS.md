# AGENTS.md

This file is the local handoff guide for agents working inside `kernel-oss`.

Scope:

- Only use and reference files inside this repository.
- Do not rely on adjacent repositories or directories for task execution guidance from this file.

## Primary Local References

Read these first:

1. `1-plans/current-handoff.md`
2. `1-plans/rust-engineering-standards-delta.md`
3. `README.md`
4. `Cargo.toml`

For seam and gateway work, inspect:

- `src/usecase/mod.rs`
- `src/gateway/mod.rs`
- `src/gateway/new_identity/mod.rs`
- `src/gateway/current_utc_timestamp/mod.rs`
- `src/gateway/directory_list/mod.rs`
- `src/gateway/file_data_gateway/mod.rs`
- `src/gateway/logger/mod.rs`

For verification patterns, inspect:

- `src/usecase/tests.rs`
- `src/gateway/tests.rs`
- `src/gateway/new_identity/tests.rs`
- `src/gateway/current_utc_timestamp/tests.rs`
- `examples/gateway_usecase_composition.rs`
- `examples/async_gateway_usecase_composition.rs`
- `examples/unit_success_payload.rs`

## Current Local Direction

- True no-input seams use:
  - `VoidUseCase` or `AsyncVoidUseCase`
  - `VoidGateway` or `AsyncVoidGateway`
- Request-bearing seams use:
  - `UseCase` or `AsyncUseCase`
  - `Gateway` or `AsyncGateway`
- Domain-specific marker seams stay as thin supertraits over the shared roles.
- For foundational kernel seams, prefer paired sync and async marker seam variants when both execution models are reasonable.
- Preserve compatibility when retrofitting legacy public gateway types or traits:
  - add a side-by-side standards-aligned seam first
  - keep the old API temporarily
  - add tests before deprecation guidance

## Current Recommended Next Task

Retrofit `RetrieveDirectoryPath` next.

Current file:

- `src/gateway/directory_list/mod.rs`

Recommended approach:

1. Add a side-by-side replacement module.
2. Introduce a bounded request object instead of bare `&str`.
3. Add a sync marker seam over `Gateway`.
4. Add an async marker seam over `AsyncGateway` if the capability should remain available to async callers.
5. Keep the existing alias temporarily for compatibility.
6. Add standards-aligned tests before deprecating the old alias.

## After That

1. Retrofit `FileDataGateway`.
2. Design the `Logger` migration separately before implementing it.
3. Then return to:
   - the `ULID` module move
   - the `src/algorithms` extraction plan
   - remaining docs, tests, and clippy cleanup

## Local Verification Commands

Run from the repository root:

```text
cargo fmt --all
cargo test
```

Use these when relevant:

```text
cargo check --examples
cargo run --example gateway_usecase_composition
cargo run --example async_gateway_usecase_composition
cargo run --example unit_success_payload
```

## Working Rules For This Repo

- Use the current shared seam direction already implemented in `src/usecase/mod.rs` and `src/gateway/mod.rs`.
- Keep marker seams explicit in examples and tests with fully qualified shared-trait execution where seam visibility matters.
- Do not remove compatibility APIs until there is an explicit breaking-change plan.
- Update `1-plans/current-handoff.md` and `1-plans/rust-engineering-standards-delta.md` when you materially change the current stopping point or next recommended work.
