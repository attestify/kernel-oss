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

## Verification commands

Run all of the following from the repository root. Each command must exit
successfully without changing generated files. These offline checks verify
custody and reproduction; they do not replace the Rust conformance tests or
release baseline in [Test architecture](../../docs/testing.md).

```text
python3 tools/url-data/verify_custody.py
python3 tools/url-data/generate_idna_tables.py --check
python3 tools/url-data/generate_idna_vectors.py --check
python3 tools/url-data/generate_p6_url_qualification_vectors.py --check
python3 tools/url-data/generate_urltestdata_vectors.py --check
python3 tools/url-data/generate_wpt_idna_v2_vectors.py --check
python3 tools/url-data/generate_wpt_idna_vectors.py --check
```

Run generators without `--check` only when intentionally regenerating outputs
for a reviewed source-data or generator change. Preserve the pinned inputs and
their custody manifests.
