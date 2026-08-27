# AKNL URL P2 private IDNA engine

P2 adds a dependency-free, crate-private Unicode 17 IDNA engine beneath the
existing URL module. It has no public URL or RepositoryLink surface and is not
wired into host parsing; P3 remains responsible for that integration.

The private engine separates raw nontransitional UTS #46 ToASCII/ToUnicode from
the WHATWG domain wrapper. The raw path preserves ContextJ, bidi, A-label, and
DNS-length validation. The relaxed WHATWG wrapper keeps its required all-ASCII
lowercase compatibility path, then rejects empty results and the WHATWG
forbidden-domain code-point set. This specifically preserves the distinct
`xn--ab-j1t` strict/relaxed outcomes.

`tools/url-data/generate_idna_tables.py` deterministically generates passive
tables from approved Unicode 17.0.0 IDNA mapping, joining-type, bidi-class, and
general-category inputs. The engine reuses the existing crate-private NFC
normalizer and combining-class table through its minimal virama query; it adds
no NFC table, normalizer, or generator.

The suite exercises 30 private IDNA tests, 87 WPT `toascii.json` cases, 2,669
Rust-representable WPT `IdnaTestV2.json` records (two ill-formed UTF-16 records
are not representable by Rust `&str`), and 1,022 directly asserted Unicode 17
UTS #46 vectors (661 success and 361 error records). A further 5,367
status-bearing ToUnicode-only records remain in the fixture census for the
separate ToUnicode projection.
The empty WPT `IdnaTestV2` conversion is checked at the pre-wrapper layer; the
final wrapper rejects it. No substantive unresolved behavior remains.

The resource policy imposes no arbitrary input limit, does no runtime I/O or
recursion, uses checked Punycode arithmetic, and performs strict DNS length
validation only when requested. Numerical performance limits remain P6 work.
