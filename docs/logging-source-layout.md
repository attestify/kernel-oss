# Operational Logger source layout

This architecture reference maps source annotation Layout IDs to their owning
files and design targets. Paths are repository-relative; Kernel OSS is the root
Cargo project. Design identity `kernel-oss-operational-logger@0.1.0` identifies a
document revision, not the Kernel crate version or release tag.

See the [consumer contract](logging.md), [design targets](logging-design.md#verification-targets)
and [test architecture](testing.md) for behavior and verification rules.

## Public module structure

`src/lib.rs` declares public `logging`. Its barrel privately declares `seam` and
`no_op_logger`, then publicly re-exports `Logger` and `NoOpLogger`. The canonical
consumer paths are `kernel_oss::logging::{Logger, NoOpLogger}`. No root-level
alias, public implementation submodule or duplicate trait is provided.

The seam is declaration-only. The no-op has a private unit field, infallible
`new`, delegating `Default` and explicit empty methods. Its colocated tests use
the ordinary public contract. Legacy Logger and WriteLogEntry implementations
retain their existing executable contracts; only preferred-use guidance changes.

## Source responsibility map

Layout IDs remain stable because source comments refer to them. They identify
source responsibilities, not delivery tasks or approval status.

| Layout ID | Path | Responsibility | Design targets |
| --- | --- | --- | --- |
| F01 | `src/lib.rs` | Root barrel | LOG-T09,LOG-T12 |
| F02 | `src/logging/mod.rs` | Barrel | LOG-T09 |
| F03 | `src/logging/seam.rs` | Unversioned declaration-only flat leaf | LOG-T05,LOG-T08,LOG-T09,LOG-T11 |
| F04 | `src/logging/no_op_logger/mod.rs` | Concrete behavior | LOG-T01–LOG-T07 |
| F05 | `src/logging/no_op_logger/tests.rs` | Focused tests | LOG-T01–LOG-T07 |
| F06 | `tests/logging_consumer.rs` | External consumer target | LOG-T01,LOG-T05,LOG-T09 |
| F07 | `tests/logging_compile.rs` | Compiler/subprocess harness | LOG-T02–LOG-T04,LOG-T07–LOG-T09 |
| F08 | `tests/fixtures/logging/positive.rs` | Positive binary | LOG-T01–LOG-T04,LOG-T07–LOG-T09 |
| F09 | `tests/fixtures/logging/missing_method.rs` | Negative binary root | LOG-T08 |
| F10 | `tests/fixtures/logging/non_send.rs` | Negative binary root | LOG-T08 |
| F11 | `tests/fixtures/logging/non_sync.rs` | Negative binary root | LOG-T08 |
| F12 | `tests/fixtures/logging/extra_initialize.rs` | Negative binary root | LOG-T08 |
| F13 | `tests/fixtures/logging/result_return.rs` | Negative binary root | LOG-T08 |
| F14 | `tests/logging_support/mod.rs` | Test barrel | LOG-T05 |
| F15 | `tests/logging_support/value/mod.rs` | Test type barrel | LOG-T05 |
| F16 | `tests/logging_support/value/recording_logger/mod.rs` | Test-only implementation | LOG-T05 |
| F17 | `README.md` | Consumer navigation | LOG-T09,LOG-T11,LOG-T12 |
| F18 | `src/gateway/logger/mod.rs` | Legacy docs | LOG-T10,LOG-T11 |
| F19 | `src/gateway/write_log_entry/mod.rs` | Compatibility docs | LOG-T10,LOG-T11 |
| F20 | `src/gateway/logger/tests.rs` | Legacy evidence | LOG-T10 |
| F21 | `src/gateway/write_log_entry/tests.rs` | Gateway evidence | LOG-T10 |
| F22 | `docs/kernel-catalog.md` | API catalog | LOG-T09–LOG-T12 |
| F23 | `docs/testing.md` | Test ownership guidance | LOG-T08,LOG-T11,LOG-T12 |
| F24 | `docs/logging.md` | Consumer contract and local diagnostic policy | LOG-T01–LOG-T12 |
| F25 | `Cargo.toml` | Package identity | LOG-T09,LOG-T12 |
| F26 | `tests/fixtures/logging/value/missing_method/mod.rs` | Test-only negative implementation | LOG-T08 |
| F27 | `tests/fixtures/logging/value/non_send/mod.rs` | Test-only negative implementation | LOG-T08 |
| F28 | `tests/fixtures/logging/value/non_sync/mod.rs` | Test-only negative implementation | LOG-T08 |
| F29 | `tests/fixtures/logging/value/extra_initialize/mod.rs` | Test-only negative implementation | LOG-T08 |
| F30 | `tests/fixtures/logging/value/result_return/mod.rs` | Test-only negative implementation | LOG-T08 |
| F31 | `Cargo.lock` | Root resolution | LOG-T09,LOG-T12 |
| F32 | `tests/usecase_event_handler_compile.rs` | Existing compiler regression | LOG-T12 |
| F33 | `tools/test-layout/verify_test_coverage_baseline.py` | Discovery guard | LOG-T12 |

## Test and fixture boundaries

- `src/logging/no_op_logger/tests.rs` verifies the actual no-op, including repeated
  and concurrent calls for LOG-T06. Source inspection establishes absence of
  allocation, retained input and other effects beyond observable stdout/stderr.
- `tests/logging_consumer.rs` verifies explicit construction and exact public
  dispatch/sharing under LOG-T01/05/09. Its recording fixture is not no-op evidence.
- `tests/logging_support/mod.rs` and its `value/mod.rs` are private test barrels.
  The dedicated RecordingLogger home supplies ordinary constructor/snapshot
  access and implements the actual public trait; production private state is
  never exposed to tests.
- `tests/logging_compile.rs` builds the positive fixture against the actual root
  library, executes it directly to check successful silent output, then checks
  missing method, independent Send/Sync, unsupported initialize and Result-return
  failures. Each negative binary root declares its separately owned fixture type.
- The compiler workspace, manifest, independent lock and build output live under
  `target/logging-contract-<process-id>/` and are removed after execution, including
  harness assertion failure. They do not replace root locked-dependency checks.

No production backend, worker, queue, initialization hook, diagnostic API type or
additional dependency belongs in this module. Error/provenance, Handler and
unrelated Usecase/Gateway behavior remain outside its implementation boundary.
