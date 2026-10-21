# Kernel OSS Test Architecture

## Purpose

Kernel OSS separates focused unit verification from exhaustive conformance and
cross-component integration verification. Test placement follows the boundary
being exercised, not the number of cases in a table.

This document is the durable control for reorganizing the NFC and URL test
suites. The reorganization may change paths, imports, and test-harness wiring,
but it must not remove a behavior, assertion, fixture record, generated vector,
or public diagnostic check.

## Test tiers

### Focused unit tests

Focused unit tests remain beside the source module under `src/**/tests.rs`.
Each test covers one bounded behavior of that module, such as a constructor
invariant, parser transition, encoding boundary, error path, or redaction
contract. Private helpers are tested directly only when the behavior cannot be
observed precisely through a public interface.

### Private component conformance tests

Some standards behavior is intentionally private but still needs exhaustive
qualification. Examples include raw UTS #46 outcomes, ContextJ and bidi
validation, Punycode primitives, internal parser validation conditions, and
private record invariants.

These suites use the library test harness so they can reach crate-private
seams, but they belong in explicitly named `conformance_tests.rs` modules
rather than being mixed into focused `tests.rs` modules. Moving these suites
must not make a private engine, table, record, or parser state public.

### Public integration and conformance tests

Tests under `tests/` exercise Kernel OSS as an external Rust consumer. They
cover complete public behavior across modules, including:

- Unicode NFC qualification through `NfcText`;
- WHATWG URL parsing, canonical serialization, resolution, and mutation
  through `URL` and `URLMutator`;
- RFC 3986 resolution compatibility;
- frozen URL compatibility-oracle projections;
- `RepositoryLink` and `Procedure` interactions;
- public error classification and exact redacted diagnostics; and
- behavior that must remain unavailable at the public API boundary.

An integration test may not import or expose a private engine merely to make
the move convenient.

Usecase Event Handler consumer tests exercise public typing, trait-object
dispatch, explicit no-op bindings and unchanged delivery Errors.
`tests/usecase_event_handler_compile.rs` runs the compiler fixtures under
`tests/fixtures/usecase_event_handler/` against this actual library.
Its separate Domain crate owns the markers and a permitted no-op binding.
A separate consumer rejects a foreign marker implementation for the Kernel
no-op with `E0117`; a local marker inside an integration test alone cannot
prove that restriction.

The compiler suite first compiles and runs its positive control, then checks
the intended diagnostic codes for wrong inputs, missing associated types or
base implementations, distinct marker identities, and the no-op's `Send + Sync`
bounds. Generated manifests, lockfile and isolated build output stay beneath
`target/handler-contract-<process-id>/` and are removed after the test.
The harness requires Cargo and the current Rust toolchain, runs offline and
adds no dependency. It runs with `cargo test --all-targets --locked --offline`
or directly with:

```text
cargo test --test usecase_event_handler_compile --locked --offline
```

The nested fixture workspace generates its own temporary lockfile against
the current source path. It does not change the repository lockfile or replace
normal locked dependency verification.

Operational Logger behavior is tested in
`src/logging/no_op_logger/tests.rs`. `tests/logging_consumer.rs` exercises public
imports, method dispatch and trait-object sharing, using the ordinary recording
fixture in `tests/logging_support/value/recording_logger/mod.rs`; the support
and value `mod.rs` files are declaration/re-export barrels.

`tests/logging_compile.rs` builds fixtures under `tests/fixtures/logging/` against
the actual library path. `positive.rs` must build and execute silently. The
`missing_method.rs`, `non_send.rs`, `non_sync.rs`, `extra_initialize.rs` and
`result_return.rs` binary roots load their corresponding implementations from
`value/<fixture-name>/mod.rs`. Each negative must fail for its intended obligation:
E0046 (missing required method), E0277 (Send or Sync independently), E0407
(unsupported initialize method), or E0053 (incorrect Result return).

Temporary manifests, lockfiles and build output reside under
`target/logging-contract-<process-id>/` and are cleaned up, including on harness
assertion failure. The nested workspace runs offline and does not replace the
repository's locked dependency checks. The positive binary is executed directly,
so compiler output cannot be mistaken for Logger output. Run the suite with:

```text
cargo test --test logging_compile --locked --offline
```

Format/check these fixtures explicitly with Rust edition 2024; ordinary Cargo
formatting need not discover intentionally invalid fixture implementations.
Preserve the discovery baseline below; new tests do not replace its obligations.

### Generator tests and checks

Generator unit tests remain under `tools/`. They verify parsing, deterministic
rendering, pinned-input censuses, checksum rejection, and checked-in output
reproduction. Generator `--check` commands prove that approved inputs still
produce the committed test vectors and private tables.

### Performance tests

The URL performance workload remains a non-product Cargo benchmark. It is not
executed by the ordinary all-target test command and does not substitute for
behavioral tests.

## Discovery baseline and current layout

The accepted pre-migration baseline was:

| Harness | Baseline | Meaning |
| --- | ---: | --- |
| Rust crate test harness | 485 | Unit, private-component, URL facade, and consumer tests |
| Rust integration harness | 8 | Public Unicode 17 `NfcText` conformance tests |
| Rust doctests | 30 | 15 compile-pass and 15 compile-fail public API checks |
| Python generator unit tests | 18 | 5 NFC generator tests and 13 URL/IDNA generator tests |

After migration, the same 493 Rust tests are discovered as 478 library/private
tests and 15 public integration tests. Doctest and Python counts remain 30 and
18 respectively. The change in harness distribution is intentional; the total
behavior floor and stable names are unchanged.

The exact 164 Rust tests in the NFC, URL, `RepositoryLink`, and `Procedure`
slice are recorded in [`tests/test-coverage-baseline.tsv`](../tests/test-coverage-baseline.tsv).
The remaining Rust tests are outside the move, but the repository-wide Rust
test floor remains 493.

Run the discovery guard with:

```text
python3 tools/test-layout/verify_test_coverage_baseline.py
```

The guard checks test discovery rather than merely counting `#[test]` text, so
a file that Cargo no longer compiles cannot satisfy the baseline.

## Behavior and corpus baseline

### NFC

| Behavior group | Required coverage |
| --- | --- |
| Public value behavior | construction normalization, canonical equality and hashing, `is_nfc`, idempotence, boundary joining, scalar/empty preservation, canonical-only behavior, and exact redacted `Debug` |
| Private normalizer behavior | canonical decomposition, stable combining-class ordering, composition and blocking, composition exclusions, Hangul boundaries, leading non-starters, long input, and shared Virama lookup |
| Official Unicode conformance | all 20,034 Unicode 17 normalization rows and the fixed six-part census |
| Scalar qualification | all 1,112,064 valid Unicode scalar values for construction and `is_nfc` behavior |
| Fixture failure behavior | malformed structure, malformed scalars, and invalid Part 1 oracle sources fail with contextual errors |
| Generation | Unicode input checksums, source censuses, deterministic table reproduction, canonical-only exclusions, and malformed-input rejection |

### URL and IDNA

| Behavior group | Required coverage |
| --- | --- |
| Public URL facade | canonical construction and accessors, absent versus present-empty components, immutable resolution, immutable one-shot mutation, last-setter-wins behavior, equality/hash, host boundary protection, exact redacted errors/`Debug`, and prohibited API/source surfaces |
| WHATWG parser and serializer | 891 pinned WPT URL rows: 624 expected successes and 267 expected failures |
| Canonical record stability | all 624 successful WPT records satisfy parse/serialize record equality |
| RFC resolution | all 42 RFC 3986 section 5.4 normal and abnormal reference vectors |
| Frozen compatibility oracle | all 190 records, including 144 successful public parses, 46 expected failures, query-pair projections, security canaries, and all recorded `RepositoryLink` outcomes |
| URL mutation qualification | all 278 WPT setter rows remain classified: 127 mapped, 137 immutable-ABI divergences, and 14 browser-only cases |
| UTS #46 | all 6,391 Unicode 17 source records remain accounted for: 661 ToASCII successes, 5,551 ToASCII failures, 177 ToUnicode-only outcomes, and 2 unrepresentable UTF-16 exclusions |
| WPT IDNA | 87 `toascii.json` rows and 2,669 Rust-representable `IdnaTestV2.json` rows, with 2 recorded ill-formed UTF-16 exclusions |
| IDNA rules | mapping, NFC reuse, A-label processing, ContextJ, bidi, STD3, hyphen, DNS length, forbidden code points, strict/relaxed wrappers, and checked Punycode |
| Host processing | special and opaque hosts, IPv4 number forms and validation, IPv6 parsing/serialization and embedded IPv4, IDNA conditions, forbidden units, and long/overflow inputs |
| Encoding | percent encode-set boundaries, one-pass decoding, all 7 pinned UTF-8 fixture rows, and ordered form pairs including duplicates and empty names/values |
| Private record invariants | authority/host/path compatibility, canonical component payloads, file/drive behavior, opaque-path stability, delimiter distinctions, and serialize/parse equality for every accepted representative shape |
| Consumer behavior | all focused `RepositoryLink` and `Procedure` success/error paths, default/allowed schemes, complete contained URL projections, and redacted consumer diagnostics |

## Data ownership and locations

Behavioral fixtures and generation inputs have different owners.

| Data | Role | Authoritative location |
| --- | --- | --- |
| `NormalizationTest-17.0.0.txt` | Behavioral NFC oracle | `tests/data/nfc/unicode/` |
| `UnicodeData-17.0.0.txt` | NFC table-generation input | `tools/unicode-nfc/data/` |
| `DerivedNormalizationProps-17.0.0.txt` | NFC table-generation input | `tools/unicode-nfc/data/` |
| WPT JSON fixtures | Behavioral URL/IDNA fixtures | `tests/data/url/wpt/` |
| Frozen URL oracle | Behavioral compatibility oracle | `tests/data/url/oracle/` |
| `IdnaTestV2.txt` | Behavioral UTS #46 oracle | `tests/data/url/unicode/` |
| IDNA Unicode property files | Private IDNA table-generation inputs | `tools/url-data/data/unicode/` |
| WPT license and behavioral custody metadata | Fixture custody | `tests/data/url/` |

Generators, include paths, custody manifests, documentation, and Cargo package
rules must preserve these ownership boundaries. No fixture may have two
authoritative copies.

## Migration controls

The move is accepted only when all of the following are true:

1. Every row in `tests/test-coverage-baseline.tsv` is still discovered. Tests
   approved to move may change their qualified module path, but retain their
   behavior-focused function name. Tests approved to stay must retain their
   complete qualified name.
2. The repository-wide Rust test count is not below 493, the doctest count is
   not below 30, and all 18 generator unit tests remain discoverable.
3. Every corpus count in this document is asserted by an executing test; a
   fixture file merely existing does not count as coverage.
4. Existing NFC and URL checksum/custody manifests remain authoritative and
   are updated for paths without changing approved fixture bytes.
5. Every generator reproduces its committed output in `--check` mode before
   and after `cargo fmt --all --check`.
6. The full Rust, doctest, generator, package, and release-build commands pass.
7. `git diff --find-renames` shows moves and the bounded import/path changes.
   Any removed assertion or test function is a blocker.

Source-body hashes are not the controlling test identity. Moving a test from a
private module to the public integration boundary necessarily changes imports
and often the call site. Git already preserves exact source history, existing
custody manifests preserve fixture identity, and execution plus the stable
behavior ledger proves discovery and behavior. Adding another hierarchy of raw
evidence hashes would duplicate those controls without improving the result.

No test may be removed as duplicate during the layout migration. A later
deduplication proposal, if any, requires a separate behavior-by-behavior review
after the reorganized suite is accepted.
