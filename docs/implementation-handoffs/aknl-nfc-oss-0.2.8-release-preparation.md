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

The locked/offline qualification passed strict clippy, 381 unit tests, 8 NFC
integration tests, 17 doctests, and 5 generator tests. The public compile
check confirms the type path, `Value` access, `Send + Sync`, normalization, and
exact redacted `Debug` output. Generator output reproduced twice and matched
the committed 109,191-byte table with SHA-256
`7189aab455a06fa193b001a6d72fd8cc4033f21f60f8b46f960a68346c4ac254`.

The final package snapshot reconciles 329 `cargo package --list` paths with
329 `.crate` archive paths and contains zero AppleDouble, `.DS_Store`, Python
cache, or `.pyc` artifacts. NFC source, Unicode inputs, generator assets,
conformance test, notices, and both license files are present.

Three evidence-capture preflights are retained as failures: two package-list /
archive comparisons where an in-package raw-output file was created between
the commands, and one temporary Rust file with an invalid inferred crate name.
They are superseded by the external-output package snapshot and explicit
crate-name public compile check; none reflects a product or package failure.

The release evidence ledger records 43 passing commands and 4 retained failed
preflights. The 49-entry detached evidence checksum manifest has SHA-256
`7207aaaa9cb9ae6fdb6e890d6c6abbf04009e783022b6b5b770972a6553e3b26`
and verifies successfully. The qualification commit that records these files
is local only and remains subject to release review; it does not create the
`0.2.8` tag.
