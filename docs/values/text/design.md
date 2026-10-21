# NFC text design and maintenance

[Usage guide](README.md) · [Documentation index](../../README.md)

## Shared private normalization

Kernel OSS owns one crate-private NFC engine and one generated table set. Both
`NfcText` and the private URL/IDNA engine use that same capability. There is no
second normalizer and no public normalization engine, iterator, quick-check
surface, table, or generator API.

The normalizer performs canonical decomposition, canonical combining-class
ordering, and canonical composition with deterministic, in-memory processing.
It performs no runtime I/O or network access.

## Data and source custody

The normative behavioral fixture is stored under `tests/data/nfc/unicode/`,
while generator-only inputs are owned by `tools/unicode-nfc/data/`:

- `tests/data/nfc/unicode/NormalizationTest-17.0.0.txt` supplies official
  conformance cases;
- `tools/unicode-nfc/data/UnicodeData-17.0.0.txt` supplies canonical combining
  classes and decompositions; and
- `tools/unicode-nfc/data/DerivedNormalizationProps-17.0.0.txt` supplies
  composition exclusions.

`tools/unicode-nfc/generate_tables.py` is the single offline generator. Its
checksum file binds the approved inputs and `--check` verifies the committed
private table byte-for-byte.

The private algorithm retains the recorded MIT-licensed influence from
`unicode-normalization` 0.1.25. Unicode data is retained under Unicode License
v3. Complete attribution and license texts are in
[THIRD_PARTY_NOTICES.md](../../../THIRD_PARTY_NOTICES.md) and `third-party/licenses/`.

## Verification

```text
python3 -m unittest tools/unicode-nfc/test_generate_tables.py
python3 tools/unicode-nfc/generate_tables.py --check
cargo test --test nfc_text_unicode_17_conformance --locked --offline
```

The conformance test exercises the official Unicode normalization corpus and
the public `NfcText` boundary. Ordinary all-target and doctest commands remain
the release gates described in the repository README.
