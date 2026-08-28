# Unicode 17.0 NFC table generator

`generate_tables.py` is a standard-library-only, offline generator for the
Kernel OSS private `NfcText` NFC tables. It verifies the exact committed Unicode 17.0
data and license checksums before parsing only canonical decomposition,
non-zero canonical combining-class, and permitted canonical composition data.

The committed inputs have distinct roles. `UnicodeData-17.0.0.txt` supplies
the canonical combining classes and canonical decompositions used by offline
generation. `DerivedNormalizationProps-17.0.0.txt` supplies the
`Full_Composition_Exclusion` property that constrains canonical composition.
`NormalizationTest-17.0.0.txt` is the normative behavioral oracle consumed by
the private-engine tests and public conformance test, not by table
generation. `checksums.sha256` binds every committed input, and `--check`
proves that the checked-in output remains exact.

Run `python3 tools/unicode-nfc/generate_tables.py --check` to verify the
checked-in table is current. Run without `--check` only when the authorized
Unicode source data changes and the generated output must be refreshed.

The generator does not access the network, update Cargo metadata, or expose a
public API. Its normalizer retains the recorded MIT attribution and the Unicode
inputs retain Unicode License v3 custody. Its test suite is run with:

```text
python3 -m unittest tools/unicode-nfc/test_generate_tables.py
```
