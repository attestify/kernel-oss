# URL design and maintenance

[Usage guide](README.md) · [Documentation index](../../../README.md)

## Private engine architecture

One private engine serves construction, resolution, and mutation:

- parser and resolver produce one canonical record;
- serializer produces the public canonical value;
- host dispatch separates domains, IPv4, IPv6, and opaque hosts;
- private IDNA performs Unicode 17 UTS #46 and checked Punycode processing;
- percent-encoding and form-query modules own byte/component transformations;
- IDNA reuses the single crate-private NFC normalizer; and
- public errors are translated at the facade boundary.

The engine is deterministic, performs no runtime I/O, uses no input-controlled
recursion, and exposes none of its records, parser states, host types, IDNA
types, NFC types, or generated tables publicly. See the
[architecture diagram](architecture.puml) for the component relationships.

## Normative profile and precedence

The implementation is standards-first. When authorities differ, the pinned
WHATWG URL behavior controls URL parsing, records, hosts, serialization,
resolution, percent encoding, and form encoding. WHATWG Infra and Encoding
control the algorithms they supply; Unicode controls IDNA and normalization;
the RFCs below provide supplemental protocol, algorithm, and qualification
context. An RFC interpretation does not override an observable WHATWG URL
result.

| Authority | Frozen revision | Role |
| --- | --- | --- |
| [WHATWG URL](https://github.com/whatwg/url/commit/55d6699373ba68a16ec182f34222a74ed8bc3dac) | `55d6699373ba68a16ec182f34222a74ed8bc3dac` | Controlling URL algorithms |
| [WHATWG Infra](https://github.com/whatwg/infra/commit/3f984adcd24a6d5c53cc26b3e737701808003f3e) | `3f984adcd24a6d5c53cc26b3e737701808003f3e` | Shared string, scalar, list, and code-point algorithms |
| [WHATWG Encoding](https://github.com/whatwg/encoding/commit/67494fceee1b95bbc87b58142fcc14e48310f257) | `67494fceee1b95bbc87b58142fcc14e48310f257` | UTF-8 encode/decode behavior |
| [Unicode UTS #46](https://www.unicode.org/reports/tr46/tr46-35.html) | Unicode 17, revision 35 | Nontransitional IDNA compatibility processing |
| [Unicode UAX #15](https://www.unicode.org/reports/tr15/tr15-57.html) | Unicode 17, revision 57 | Shared NFC normalization |
| [Unicode UAX #44](https://www.unicode.org/reports/tr44/tr44-36.html) | Unicode 17, revision 36 | Unicode Character Database property interpretation |

The supplemental RFC profile includes RFC 3492 for checked Punycode; RFC
5890–5893 for IDNA terminology, code-point, ContextJ, and bidi rules; RFC 3986
and RFC 3987 for URI/IRI context and the section 5.4 resolution vectors; RFC
4291 and RFC 5952 for IPv6 context; RFC 791, RFC 1122, RFC 1123, RFC 1034, and
RFC 1035 for IPv4, host, and optional DNS-length context; and RFC 8089 for
`file:` URI context. The WHATWG algorithms remain controlling where those
documents differ.

The IDNA engine uses nontransitional mapping, always checks ContextJ and bidi,
and uses the shared NFC engine. Strict processing additionally applies STD3
ASCII, hyphen, and DNS-length checks. The URL domain wrapper preserves the
specified relaxed ASCII compatibility behavior while still rejecting empty
and forbidden results. Punycode arithmetic is checked. Legacy encoding
overrides, DNS lookup, IPv6 zone identifiers, and a public origin API are not
part of this contract.

## Source and data custody

The URL implementation is Attestify-owned, independently structured from the
standards above, and contains no external URL implementation source. The former
`url` 2.5.7 package was executed only through its public API to produce the
frozen compatibility oracle. It is not a dependency, implementation source,
public type source, or packaged artifact.

Pinned behavioral fixtures and the oracle live under `tests/data/url/`.
[`tests/data/url/SOURCES.tsv`](../../../../tests/data/url/SOURCES.tsv) records the exact
WPT commit, Unicode version, source paths, roles, and SHA-256 values;
[`tests/data/url/CHECKSUMS.sha256`](../../../../tests/data/url/CHECKSUMS.sha256) binds
their bytes. Generator-only Unicode inputs are separately owned by
`tools/url-data/data/`, with their own source and checksum manifests.
[`tools/url-data/README.md`](../../../../tools/url-data/README.md) defines deterministic,
offline reproduction and custody verification. Licenses and attribution are
retained in [`THIRD_PARTY_NOTICES.md`](../../../../THIRD_PARTY_NOTICES.md) and the
corresponding data directories.

A WHATWG, WPT, Unicode, or IDNA-profile update is a reviewed behavioral change.
It must update the frozen revision, source and checksum manifests, generated
tables/vectors, conformance counts, compatibility classifications, notices,
and performance qualification together. A partial version update is not an
accepted state.

## Verification

Run the complete [offline custody and generator checks](../../../../tools/url-data/README.md#verification-commands)
from the repository root to verify all generated tables and vectors.

The permanent suite covers URL parsing, canonical round trips, resolution,
setters, IDNA, percent/form processing, host parsing, redacted diagnostics,
adversarial inputs, and release-mode performance. Exact behavioral floors,
corpus counts, exclusions, data ownership, and the discovery guard are defined
in [Kernel OSS Test Architecture](../../../testing.md). Run the normal crate checks from
the README; run the dedicated benchmark with:

```text
cargo bench --bench kernel_oss_url_performance --locked --offline
```
