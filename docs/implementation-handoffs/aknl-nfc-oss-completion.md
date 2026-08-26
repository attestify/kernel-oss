# AKNL-NFC-OSS-P1 bounded NFC relocation completion

Outcome: **REVIEW_REQUIRED**

## Baselines

- Kernel OSS source baseline: `27ebd5f638e7a654c81d58831adbeef006613cc1`
- Kernel Proprietary read-only source: `e2803b1c21320daa2647e1c23cd29cd0cb4609d8`
- Engineering standards: `13b770f0347a96b06c58d061ed40608afbe8d31a`

Kernel Proprietary remained clean and read-only. The original relocation
candidate HEAD was `98cb0370e039ece840d5192124b4fa5b05f9fa04`; the exact
review-round correction commit and resulting HEAD are recorded in the local
commit history returned with this packet. This completion record is updated by
that correction commit and does not claim an uncommitted worktree is clean.

The pre-correction local commit series was:

- `68bf8c6` docs(kernel-oss): authorize NFC relocation
- `678950c` chore(kernel-oss): add NFC custody and generation assets
- `df28d0f` feat(kernel-oss): relocate NfcText and shared normalizer
- `ba07734` test(kernel-oss): qualify NFC public boundary
- `98cb037` docs(kernel-oss): record NFC relocation evidence

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
- Fifteen exhaustive runs passed with immutable per-run UTC start/completion,
  command, raw-output path, and SHA-256 evidence. The current median is 1.65
  seconds and maximum is 2.19 seconds, within the 15-second median and
  30-second maximum review bounds.
- The current full suite passed: 381 unit tests, 8 NFC integration tests, and
  17 doctests. Generator tests passed 5/5.
- Formatting, all-target tests, doctests, generator check, generator tests,
  package build, and package listing passed. The current clippy result remains
  the unrelated baseline finding described below.

`evidence-register.tsv` classifies every retained evidence artifact as ACTIVE,
SUPERSEDED, FAILED_PREFLIGHT, or SUPPORTING; it preserves the historical
exit-101 conformance preflight and its cause rather than presenting it as a
passing run. `active-command-evidence.tsv` and
`R1-ACTIVE-COMMAND-HASH-VERIFY-001.out` verify every current command-evidence
row and recorded SHA-256 value.

## Range-diff and repository integrity

The required full-range command
`git diff --check 27ebd5f638e7a654c81d58831adbeef006613cc1..HEAD` exits 2
because preserved raw evidence files `TEST-ALL-TARGETS-003.out` and
`TEST-ALL-TARGETS-004.out` each contain a trailing blank line. Raw evidence
was deliberately not edited. The evidence-excluded command,
`git diff --check 27ebd5f638e7a654c81d58831adbeef006613cc1..HEAD -- .
':(exclude)docs/implementation-handoffs/aknl-nfc-oss-evidence/*.out'`, passes.

`git fsck --no-dangling` exits 8. Its raw output records 164 `.git/._*`
AppleDouble files, including `.git/refs/heads/._main` and object sidecars.
Git reports invalid ref/object storage metadata. No `.git` file was modified,
and the provenance of these filesystem metadata files relative to the original
baseline cannot be established through Git history. Repository-storage
integrity is therefore **OPEN**, notwithstanding a clean tracked worktree.

## Package-archive hygiene

The rebuilt archive inventory has 268 entries, including three `.DS_Store`
entries (`src/.DS_Store`, `src/gateway/.DS_Store`, and
`src/values/.DS_Store`) and 14 AppleDouble `._*` entries. The separately
captured `cargo package --list --allow-dirty` output has 281 entries, including
the same three `.DS_Store` entries and 28 AppleDouble entries. The two listings
do not reconcile because AppleDouble metadata changed between the package-list
and package-archive invocations. No package rule or filesystem metadata file
was changed under this correction authorization.

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
3. **HIGH — open repository-storage integrity.** `git fsck --no-dangling`
   exits 8 because of 164 `.git/._*` AppleDouble files. Cleanup is outside this
   authorization and requires separate sponsor direction.
4. **HIGH — open package-archive hygiene.** The actual `.crate` archive
   contains AppleDouble entries as well as the three existing `.DS_Store`
   entries. The archive/list mismatch and all metadata entries remain for a
   separate cleanup decision.

## Explicit boundaries

NFC behavior was relocated, not rewritten. No second NFC engine, URL source or
test change, URL P1/P2 work, Proprietary migration, dependency addition or
update, Cargo.lock change, push, tag, publication, or release occurred.
