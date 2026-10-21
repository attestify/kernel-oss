# Kernel OSS `NfcText` Unicode 17.0 Test Data

This exact committed Unicode 17.0 behavioral fixture is immutable conformance
data. Its SHA-256 value is bound by
[`tools/unicode-nfc/checksums.sha256`](../../../../tools/unicode-nfc/checksums.sha256).

| File | Role | Consumer | Verification |
| --- | --- | --- | --- |
| `NormalizationTest-17.0.0.txt` | Normative NFC behavioral oracle | private-engine tests and `tests/nfc_text_unicode_17_conformance.rs` | exact 20,034-row fixture census and public API invariants |

The generator is standard-library-only and offline. Run
`python3 tools/unicode-nfc/generate_tables.py --check` to prove the checked-in
private table is exact. Generator-only inputs are owned by
`tools/unicode-nfc/data/`. Do not modify this pinned fixture without an
explicit Unicode version and custody update.
