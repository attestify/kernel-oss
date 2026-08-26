# AKNL-NFC-OSS-P1 bounded NFC relocation completion

Outcome: **REVIEW_REQUIRED**

## Baselines

- Kernel OSS source baseline: `27ebd5f638e7a654c81d58831adbeef006613cc1`
- Kernel Proprietary read-only source: `e2803b1c21320daa2647e1c23cd29cd0cb4609d8`
- Engineering standards: `13b770f0347a96b06c58d061ed40608afbe8d31a`

Kernel Proprietary remained clean and read-only. The resulting Kernel OSS HEAD
is recorded after the authorized local commit series in the return record.

## Public ABI

`kernel_oss::values::text::NfcText` provides exactly:

- `pub fn new(text: &str) -> Self`
- `pub fn is_nfc(text: &str) -> bool`
- `Value<ValueType = str>`
- `Clone`, `PartialEq`, `Eq`, `Hash`, and a manual redacted `Debug`

Its `String` state remains private. Canonically equal NFC values compare and
hash equally. `Debug` is exactly `NfcText([REDACTED])`. No `Display`, serde,
conversion, ordering, `Default`, or `Copy` surface was added.

`PUBLIC-SURFACE-001.out` and the compile-fail rustdoc example show that
`nfc_text` and its normalization engine are not public. The engine and its two
operations are crate-private solely for future internal consumers; tables are
private to the normalizer.

## Custody and reproducibility

`source-reconciliation.tsv` maps every transferred source to its target and
hash. `adaptation-ledger.tsv` records every non-identical artifact and confirms
that no NFC algorithmic content was changed. Generated tables and all three
Unicode inputs retain their approved hashes. Two temporary generator outputs
and the committed table are each 109,191 bytes with SHA-256
`7189aab455a06fa193b001a6d72fd8cc4033f21f60f8b46f960a68346c4ac254`.

The generator census passed: 40,575 UnicodeData lines; 16,172 non-comment
DerivedNormalizationProps records; 74 Full Composition Exclusion records;
1,120 exclusion scalar values; 968 combining-class entries; 2,081
decomposition entries; and 961 composition entries.

`THIRD_PARTY_NOTICES.md` retains the Unicode License v3 custody record and the
complete Rust Project / `unicode-normalization` 0.1.25 / MIT / Attestify
modification attribution. The complete license texts are packaged under
`third-party/licenses/`.

## Qualification

- Generator unit suite: 5 passed.
- Exhaustive integration suite: 8 tests passed, covering all 20,034 official
  NormalizationTest rows and 1,112,064 valid Unicode scalars.
- Fifteen exhaustive runs passed; median 1.64 seconds and maximum 7.84
  seconds, within the 15-second median and 30-second maximum review bounds.
- Formatting, all-target tests, doctests, example check, release build,
  no-dependency docs, metadata, feature tree, generator check, deterministic
  comparison, package build, and `git diff --check` passed.

Raw output, UTC completion times, exit codes, and SHA-256 values are in
`aknl-nfc-oss-evidence/command-results.tsv`.

## Findings

1. **MEDIUM — open pre-existing clippy baseline.**
   `cargo clippy --all-targets --locked --offline -- -D warnings` fails only at
   `src/values/specification/repository_link/mod.rs:76` for
   `clippy::useless_borrows_in_formatting`. It is outside the authorized NFC
   write set and was not changed. The result requires review rather than a
   fully clean qualification claim.
2. **MEDIUM — open pre-existing package metadata.**
   `cargo package --list --allow-dirty` contains `src/.DS_Store`,
   `src/gateway/.DS_Store`, and `src/values/.DS_Store`. They are outside this
   authorization and were not changed. Required NFC notices, licenses,
   generator, inputs, checksums, and conformance test are included.

## Explicit boundaries

NFC behavior was relocated, not rewritten. No second NFC engine, URL source or
test change, URL P1/P2 work, Proprietary migration, dependency addition or
update, Cargo.lock change, push, tag, publication, or release occurred.
