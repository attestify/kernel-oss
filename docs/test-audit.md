# Kernel OSS Test Audit

Date: 2026-06-17

This audit records the current test surface inside `kernel-oss` and how it
compares to the kernel testing approach used by `test_framework_oss`.

Scope:

- audit test files under `src/**`
- compare them with the current standards in `1-plans/rust-engineering-standards-delta.md`
- identify the files that already follow the kernel helper style and the files
  that still need logical-path and documentation review

## Kernel Testing Baseline

The current repo already depends on `test_framework_oss` in `Cargo.toml`, and
the intended test style is present in several seam tests:

- `is_ok!`
- `is_error!`
- `kernel_error_eq!`
- `kernel_error_starts_with!`
- `kernel_error_contains!`

The standards-aligned direction is:

- prefer helper macros over raw `assert!(result.is_ok())` / `assert!(result.is_err())`
- prefer helper macros over `unwrap()` / `unwrap_err()` in test bodies
- name tests around the observed outcome, typically `_success` or `_error`
- keep setup expectations explicit instead of relying on ad hoc panic strings

## Test Inventory

### Baseline Complete

The helper-style conversion pass is complete across the test tree.

All test files now avoid raw `unwrap()`, `unwrap_err()`, `expect()`, and manual
`assert!(result.is_ok())` / `assert!(result.is_err())` usage in test bodies.

The required test-module documentation pass is complete across the current
`src/**` test tree.

The next audit is about logical-path coverage and test-name quality, not helper
style or missing test docs.

### Coverage Audit Targets

- `src/gateway/tests.rs`
- `src/gateway/new_identity/tests.rs`
- `src/gateway/current_utc_timestamp/tests.rs`
- `src/gateway/retrieve_directory_path/tests.rs`
- `src/gateway/file_data/tests.rs`
- `src/gateway/write_log_entry/tests.rs`
- `src/usecase/tests.rs`
- `src/error/tests.rs`
- `src/values/uri/url/tests.rs`
- `src/values/text/block/tests.rs`
- `src/values/text/line/tests.rs`
- `src/values/datetime/utc_timestamp/tests.rs`
- `src/values/specification/repository_link/tests.rs`
- `src/values/datetime/start_time/tests.rs`
- `src/values/file_system/file_name/tests.rs`
- `src/values/directory/directory_list/tests.rs`
- `src/values/nrn/tests.rs`
- `src/values/nrn/filepath_codec/tests.rs`
- `src/values/specification/api_version/tests.rs`
- `src/values/specification/description/tests.rs`
- `src/values/specification/file_path/tests.rs`
- `src/values/specification/metadata/tests.rs`
- `src/values/specification/name/tests.rs`
- `src/values/specification/outcome/tests.rs`
- `src/values/specification/procedure/tests.rs`
- `src/values/specification/repository_link/tests.rs`
- `src/values/specification/short_description/tests.rs`
- `src/values/specification/subject_id/tests.rs`
- `src/values/specification/subject/tests.rs`
- `src/values/specification/assurance_procedure/action/tests.rs`
- `src/values/specification/assurance_procedure/activities/tests.rs`
- `src/values/specification/assurance_procedure/activity/tests.rs`
- `src/values/specification/assurance_procedure/artifact/tests.rs`
- `src/values/specification/assurance_procedure/artifacts/tests.rs`
- `src/values/specification/assurance_procedure/procedure/tests.rs`
- `src/values/specification/assurance_report/action/tests.rs`
- `src/values/specification/assurance_report/activities/tests.rs`
- `src/values/specification/assurance_report/activity/tests.rs`
- `src/values/specification/assurance_report/additional_information/tests.rs`
- `src/values/specification/assurance_report/signed_file/tests.rs`
- `src/values/specification/assurance_report/summary/tests.rs`
- `src/values/specification/v1_0_0/assurance_procedure/tests.rs`
- `src/values/specification/v1_0_0/assurance_report/tests.rs`

### Remaining Audit Work

The remaining audit work is not missing docs. It is deciding whether the tests
cover every logical path the standards expect for each object.

Coverage updates made during this pass:

- `DirectoryList::try_from_hashmap` success coverage added
- `DirectoryList::try_merge` duplicate-key rejection added
- `MetaData::upsert` coverage for inserting a missing key added
- `APIVersion` component accessors added
- `StartTime::time` accessor added
- `MetaData::data` accessor added
- `Subject::nrn` and `Subject::id` accessors added
- `URL` accessor coverage added for line, scheme, host, port, path, query string, query pairs, query count, and fragment
- `Summary` accessor coverage added for all summary counters and outcome
- `SignedFile::new` success and empty-path rejection coverage added
- `v1_0_0::AssuranceReportBuilder::upsert_metadata` insertion path added

## Audit Findings

1. The kernel helper approach is now established across the test tree.
2. The required test-module doc shape is complete across the current test tree.
3. The logical-path coverage audit is nearly complete for the current public
   surface and should only need a final review for any newly added APIs.
4. Test naming is now aligned to the standards suffix pattern for the current
   audited suites.

## Recommended Next Audit Order

1. Gateway and use case tests
2. Smaller value-object tests
3. `nrn`, `ulid`, and file-system helpers
4. `assurance_procedure` and `assurance_report`
5. `v1_0_0` compatibility tests

## Verification

The audit itself is informational. After the logical-path and documentation
pass, rerun:

```text
cargo test
```

and keep the helper macro usage consistent with the patterns already present in
the gateway tests.
