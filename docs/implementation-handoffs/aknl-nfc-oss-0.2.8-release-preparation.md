# AKNL-NFC-OSS-R1 local 0.2.8 release-candidate preparation

This record governs bounded local preparation of the Kernel OSS `0.2.8`
release candidate. The accepted NFC relocation base is
`34c7619eab7d067aac073a1617d5ebe0c229dc0c`; the candidate must remain its
descendant.

## Authorized scope

The only non-documentation product changes are the `kernel_oss` package and
root lockfile version from `0.2.7` to `0.2.8`, plus the behavior-neutral
RepositoryLink formatting-borrow lint correction. NFC source, tables, inputs,
public ABI, dependencies, URL/IDNA behavior, and Kernel Proprietary are out of
scope.

`NfcText` remains the public `kernel_oss::values::text::NfcText` value; its
normalizer and generated tables remain non-public implementation details.

## Release boundary

The local candidate is prepared in a clean native-filesystem clone. No `0.2.8`
tag is created, signed, or moved; nothing is pushed, published, or released.
Kernel Proprietary migration remains blocked until a separately authorized,
signed immutable OSS tag exists. URL/IDNA integration remains separately
unauthorized.

## Qualification record

Raw command output, hashes, package/archive inventories, public-surface checks,
and release diff evidence are retained under
`aknl-nfc-oss-0.2.8-release-evidence/`. The final local candidate and its
qualification outcome are returned for review before any tag or publication
authorization.

## Local candidate qualification

The local release-preparation commits are:

- `d921a94` — package and root-lockfile version preparation
- `43bad55` — the sole RepositoryLink formatting-borrow lint correction
- `edf984b` — release-preparation documentation
- `4130f10d488d5f5c6dc887085a2619162d3ab01d` — initial release-qualification record

The locked/offline qualification passed strict clippy, 381 unit tests, 8 NFC
integration tests, 17 doctests, and 5 generator tests. The public compile
check confirms the type path, `Value` access, `Send + Sync`, normalization, and
exact redacted `Debug` output. Generator output reproduced twice and matched
the committed 109,191-byte table with SHA-256
`7189aab455a06fa193b001a6d72fd8cc4033f21f60f8b46f960a68346c4ac254`.

The 329-path package/archive snapshot was valid for the earlier intermediate
candidate, but it is not final-candidate package evidence: `Cargo.toml`
includes `/docs/**`, and later evidence records changed package contents. The
final package qualification therefore occurs only after this correction is
committed, with raw output held outside this repository in the external review
packet. That packet records the exact final commit, archive reconciliation,
archive hygiene, NFC-custody presence, and `.cargo_vcs_info.json` commit.

Four immutable failed evidence-capture preflights are retained and explicitly
classified as `EXPECTED_FAILURE`, not as qualification failures:

- `RC-PACKAGE-RECONCILIATION-001` — an in-package raw output was created
  between list and archive capture; replaced by the deferred-output
  reconciliation `RC-PACKAGE-RECONCILIATION-003`.
- `RC-PACKAGE-RECONCILIATION-002` — moving the list output into the included
  documentation tree before packaging changed the archive; replaced by
  `RC-PACKAGE-RECONCILIATION-003`.
- `RC-ARCHIVE-HYGIENE-001` — a shell count treated an empty `rg -c` result as
  nonzero despite no metadata match; replaced by `RC-ARCHIVE-HYGIENE-003`.
- `RC-PUBLIC-ABI-COMPILE-001` — the temporary Rust filename produced an
  invalid inferred crate name; replaced by the explicit-crate-name
  `RC-PUBLIC-ABI-COMPILE-002`.

The intermediate `RC-LOCAL-COMMITS-001`, `RC-CHANGED-PATHS-001`,
`RC-STATUS-001`, and `RC-GIT-DIFF-CHECK-001` records are retained unchanged
but are `SUPERSEDED`: they do not describe the final correction commit. The
unqualified final base-to-HEAD whitespace check is expected to report immutable
raw `.out` evidence; the final external packet records both that result and a
passing scoped check which excludes only those raw outputs. The correction
commit and every final candidate commit are recorded in that external packet.

The in-repository evidence manifest is regenerated after this record and its
lifecycle ledgers are final. Its detached verifier is excluded from the
manifest it verifies. The qualification commits are local only and remain
subject to release review; no `0.2.8` tag is created.
