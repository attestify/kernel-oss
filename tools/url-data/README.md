# URL conformance data generation

Behavioral fixtures are stored verbatim under `tests/data/url/` with recorded
hashes. Generator-only Unicode property inputs are owned by
`tools/url-data/data/unicode/`. The frozen `url 2.5.7` oracle dataset was
generated through its public API as compatibility data and is not a product
implementation or implementation source.

`tools/url-data/generate_idna_tables.py` generates private passive IDNA tables.
The generator reads only the committed Unicode 17.0.0 IDNA mapping,
joining-type, bidi-class, and general-category inputs, verifies the approved
mapping-input hash and version, and reproduces
`src/values/uri/url/engine/idna/tables.rs` byte-for-byte. The WPT and Unicode
vector generators only turn approved fixture data into private Rust test
vectors; they do not implement URL behavior or access the network.

`verify_custody.py` verifies both authoritative data roots and their separate
checksum manifests.
