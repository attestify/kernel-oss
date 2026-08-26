# Kernel OSS `NfcText` Unicode 17.0 Test Data

These exact committed Unicode 17.0 inputs are immutable Q2 qualification and
offline-generation custody data. Their SHA-256 values are bound by
[`tools/unicode-nfc/checksums.sha256`](../../../tools/unicode-nfc/checksums.sha256).

| File | Role | Consumer | Verification |
| --- | --- | --- | --- |
| `NormalizationTest-17.0.0.txt` | Normative NFC behavioral oracle | private-engine tests and `tests/nfc_text_unicode_17_conformance.rs` | exact 20,034-row fixture census and public API invariants |
| `UnicodeData-17.0.0.txt` | Canonical combining classes and canonical decompositions | offline `generate_tables.py` | checksum verification and committed-source census |
| `DerivedNormalizationProps-17.0.0.txt` | `Full_Composition_Exclusion` property | offline `generate_tables.py` | checksum verification and committed-source census |

The generator is standard-library-only and offline. Run
`python3 tools/unicode-nfc/generate_tables.py --check` to prove the checked-in
private table is exact. These inputs were relocated from Kernel Proprietary
without changing their Unicode 17.0 contents; do not modify them without a
separate Unicode-custody authorization.
