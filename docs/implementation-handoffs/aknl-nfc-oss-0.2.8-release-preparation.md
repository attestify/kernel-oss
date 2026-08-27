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
