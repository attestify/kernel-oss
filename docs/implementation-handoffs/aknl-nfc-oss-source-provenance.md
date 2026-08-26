# AKNL-NFC-OSS-P1 source and data provenance

Kernel OSS relocated the NFC/NfcText capability from the authorized Kernel
Proprietary source baseline `e2803b1c21320daa2647e1c23cd29cd0cb4609d8`.
This is a source relocation with narrowly approved path, visibility,
documentation, fixture, generator, packaging, and integration adaptations; it
is not a clean-room implementation or algorithm rewrite.

## Source classes

- `src/values/text/nfc_text/normalization/mod.rs` retains its file-level
  attribution to `unicode-normalization` 0.1.25, commit
  `7ae3eb01d3c30da0d3c319225b306d8315ceb134`, Copyright 2012-2015 The Rust
  Project Developers, MIT license, and Attestify modification notice.
- The normalization table is generated from the approved Unicode 17 inputs.
- Unicode inputs and fixtures retain Unicode License v3 custody.
- The NfcText module, tests, generator, generator tests, and supporting
  custody records are authorized Attestify-authored relocation material.

## Canonical custody

The one canonical shared Unicode input location is `test_data/nfc/unicode/`.
The one generator location is `tools/unicode-nfc/`. The generated table remains
private under `src/values/text/nfc_text/normalization/`. Future URL/IDNA work
must privately reuse this OSS-owned engine and must not maintain a second NFC
engine or drifting Unicode copy.

## License and notice retention

`THIRD_PARTY_NOTICES.md` records the retained Unicode and MIT obligations.
Their complete texts remain in `third-party/licenses/`. The source-to-target
mapping, source hashes, adaptations, and final reconciliation are recorded in
`aknl-nfc-oss-source-authorization.md` and
`aknl-nfc-oss-completion.md`.
