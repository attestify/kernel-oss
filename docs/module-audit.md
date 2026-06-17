# Kernel OSS Module Audit

Date: 2026-06-17

This audit records how the current `kernel-oss` module layout compares with the
Rust module-organization standard in
`../../specifications/engineering-standards/7-specifications/rust/23-rust-module-organization.md`.

## Standard Baseline

The standard prefers:

- directory modules with `mod.rs` for executable bounded behavior
- sibling `tests.rs` for executable `mod.rs` modules
- barrel-only `mod.rs` files to stay free of executable behavior
- flat `.rs` leaf modules only for small passive declarations
- structural module roots that do not accumulate roadmap notes or unrelated
  implementation commentary

## Current Findings

The non-deferred module gaps identified in the first pass have been addressed
in this branch:

- `src/gateway/directory_list/mod.rs` now has colocated tests for the legacy
  alias surface.
- `src/gateway/identity/mod.rs` now has colocated tests for boxed-clone and
  execution behavior.
- `src/gateway/logger/mod.rs` now has colocated tests for the legacy logging
  trait.
- `src/gateway/utc_timestamp/mod.rs` now has colocated tests for boxed-clone
  and execution behavior.
- `src/gateway/file_data_gateway/mod.rs` now has colocated tests for the legacy
  file-data alias surface.
- `src/algorithms/os_home_directory.rs` now has colocated tests through the
  internal environment helper.
- `src/values/datetime/start_time/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/datetime/utc_timestamp/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/directory/name/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/directory/directory_list/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/file_system/file_name/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/api_version/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/description/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/file_path/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/kind/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/specification/metadata/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/name/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/specification/outcome/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/procedure/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/repository_link/` now uses a directory module
  with colocated `tests.rs`.
- `src/values/specification/short_description/` now uses a directory module
  with colocated `tests.rs`.
- `src/values/specification/subject/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/subject_id/` now uses a directory module with
  colocated `tests.rs`.
- `src/values/specification/assurance_procedure/action/`,
  `src/values/specification/assurance_procedure/activities/`,
  `src/values/specification/assurance_procedure/activity/`,
  `src/values/specification/assurance_procedure/artifact/`,
  `src/values/specification/assurance_procedure/artifacts/`,
  `src/values/specification/assurance_procedure/procedure/`,
  `src/values/specification/assurance_report/action/`,
  `src/values/specification/assurance_report/activities/`,
  `src/values/specification/assurance_report/activity/`,
  `src/values/specification/assurance_report/additional_information/`,
  `src/values/specification/assurance_report/signed_file/`,
  `src/values/specification/assurance_report/summary/`,
  `src/values/specification/v1_0_0/assurance_procedure/`, and
  `src/values/specification/v1_0_0/assurance_report/` now use directory
  modules with colocated `tests.rs`.
- `src/values/strings/` now uses a directory module with colocated `tests.rs`.
- `src/values/text/block/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/text/line/` now uses a directory module with colocated
  `tests.rs`.
- `src/values/uri/url/` now uses a directory module with colocated `tests.rs`.

### Deferred Module Boundary

1. `src/ulid/mod.rs`
   - executable root module with tests
   - contains roadmap/TODO commentary that makes the root file more than a
     structural declaration
   - should move to `src/values/ulid/` with a compatibility re-export when the
     ULID relocation work is carried out
   - deferred by request for the current pass

### Compatibility Surfaces That Are Still Acceptable

These modules are executable or compatibility-oriented, but they already have
colocated tests or are intentional compatibility surfaces:

- `src/gateway/current_utc_timestamp/mod.rs`
- `src/gateway/directory_list/mod.rs`
- `src/gateway/file_data/mod.rs`
- `src/gateway/file_data_gateway/mod.rs`
- `src/gateway/identity/mod.rs`
- `src/gateway/logger/mod.rs`
- `src/gateway/new_identity/mod.rs`
- `src/gateway/retrieve_directory_path/mod.rs`
- `src/gateway/utc_timestamp/mod.rs`
- `src/gateway/write_log_entry/mod.rs`
- `src/values/nrn/mod.rs`
- `src/usecase/mod.rs`
- `src/gateway/mod.rs`

### Broad Flat-Leaf Drift

Many bounded value and helper modules are still implemented as flat `.rs`
files with executable logic rather than directory modules. They are already
colocated with tests and behave like bounded leaf modules, but they are still a
non-preferred shape under the strict directory-first reading of the standard.

The current flat executable backlog is:

- `src/algorithms/os_home_directory.rs`
- `src/algorithms/signature_algorithm.rs`
- `src/ulid/base32.rs`

The only flat leaf that is already aligned with the passive-leaf exception is:

- `src/values/copy_value.rs`

## Recommendation

Prioritize the following module work:

1. keep `src/values/copy_value.rs` flat unless it later gains executable
   responsibility
2. defer `ULID` relocation until the future move is scheduled
3. address the algorithm split when the domain move is ready
