# Unicode 17 NFC generation inputs

These files are generator-only inputs for the crate-private NFC tables:

- `UnicodeData-17.0.0.txt` supplies canonical combining classes and canonical
  decompositions.
- `DerivedNormalizationProps-17.0.0.txt` supplies
  `Full_Composition_Exclusion`.

Their hashes are recorded in `tools/unicode-nfc/checksums.sha256`. The
behavioral `NormalizationTest-17.0.0.txt` fixture is owned by
`tests/data/nfc/unicode/`.
