# AKNL-NFC-OSS-P1 source authorization

Authorization: `AKNL-NFC-OSS-P1 — BOUNDED NFC RELOCATION AUTHORIZATION`.

Source baseline: Kernel Proprietary `main`
`e2803b1c21320daa2647e1c23cd29cd0cb4609d8`.

Target baseline: Kernel OSS `main`
`27ebd5f638e7a654c81d58831adbeef006613cc1`.

This work is an authorized relocation and narrowly bounded adaptation of the
existing NFC/NfcText implementation. It is not a clean-room effort, rewrite,
redesign, new normalizer, URL work, or Proprietary migration.

## Approved source-to-target map recorded before adaptation

| Source | SHA-256 | Target | Initial disposition |
| --- | --- | --- | --- |
| `src/values/text/nfc_text/mod.rs` | `06676e442a88dfb6bdfbdf06d928783dde6dc53e9f5332bcb20e029085217c57` | `src/values/text/nfc_text/mod.rs` | import, visibility, and documentation adaptation |
| `src/values/text/nfc_text/tests.rs` | `82536f82931aab9b13d4b1f403691351c628f9835631609656c2f05a936537ac` | `src/values/text/nfc_text/tests.rs` | documentation-path adaptation only if needed |
| `src/values/text/nfc_text/normalization/mod.rs` | `c535e6859206f82a60e163ad36e3f990eeb8c2331bd2aef4e7a053b866502cf4` | `src/values/text/nfc_text/normalization/mod.rs` | crate visibility and provenance-link adaptation |
| `src/values/text/nfc_text/normalization/tests.rs` | `43205f06f960d4294e8970ce9a61a64daeeedcb8ba533a4a00e1f3fbd49d3965` | `src/values/text/nfc_text/normalization/tests.rs` | fixture-path adaptation |
| `src/values/text/nfc_text/normalization/tables.rs` | `7189aab455a06fa193b001a6d72fd8cc4033f21f60f8b46f960a68346c4ac254` | `src/values/text/nfc_text/normalization/tables.rs` | byte-identical relocation |
| `src/values/text/nfc_text/test_data/UnicodeData-17.0.0.txt` | `2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c` | `test_data/nfc/unicode/UnicodeData-17.0.0.txt` | byte-identical relocation |
| `src/values/text/nfc_text/test_data/DerivedNormalizationProps-17.0.0.txt` | `71fd6a206a2c0cdd41feb6b7f656aa31091db45e9cedc926985d718397f9e488` | `test_data/nfc/unicode/DerivedNormalizationProps-17.0.0.txt` | byte-identical relocation |
| `src/values/text/nfc_text/test_data/NormalizationTest-17.0.0.txt` | `5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db` | `test_data/nfc/unicode/NormalizationTest-17.0.0.txt` | byte-identical relocation |
| `src/values/text/nfc_text/test_data/README.md` | `bffa21761caaebf4fa13dcaa5ea0f6f97005e1283237f7b1bdf9d1806ae3dfc3` | `test_data/nfc/unicode/README.md` | documentation and fixture-path adaptation |
| `tools/unicode-nfc/generate_tables.py` | `ab8cc0fa5792c7840e44be9fb9c794280bbfd5518e0b8f5a738f4f0686c0b7d0` | `tools/unicode-nfc/generate_tables.py` | fixture/default-path adaptation |
| `tools/unicode-nfc/test_generate_tables.py` | `87efe7c94cf2074e758a6d39a0e7e349f13249595523d5b3c13e892386af87eb` | `tools/unicode-nfc/test_generate_tables.py` | byte-identical relocation |
| `tools/unicode-nfc/checksums.sha256` | `e725604c02388d401151aad3549dc181190afa54e239e5cd2fc94c984fc8cde2` | `tools/unicode-nfc/checksums.sha256` | byte-identical relocation |
| `tools/unicode-nfc/README.md` | `0584f834f3b8cfd89c84958a201bf81a8e3883532de95ab1b9c24d71bfe1cadb` | `tools/unicode-nfc/README.md` | documentation and fixture-path adaptation |
| `tests/nfc_text_unicode_17_conformance.rs` | `8452e423835908bb0cf9c397caf93c715a5165d92c7a0e7d20c95b2b71e440c7` | `tests/nfc_text_unicode_17_conformance.rs` | crate-path and fixture-path adaptation |
| `third-party/licenses/Unicode-3.0.txt` | `e7a93b009565cfce55919a381437ac4db883e9da2126fa28b91d12732bc53d96` | `third-party/licenses/Unicode-3.0.txt` | byte-identical relocation |
| `third-party/licenses/unicode-normalization-MIT.txt` | `7b63ecd5f1902af1b63729947373683c32745c16a10e8e6292e2e2dcd7e90ae0` | `third-party/licenses/unicode-normalization-MIT.txt` | byte-identical relocation |

The relevant NFC and Unicode notices will be recreated in the OSS-root
`THIRD_PARTY_NOTICES.md`, preserving the Rust Project copyright,
`unicode-normalization` 0.1.25 attribution, MIT license, Attestify modification
notice, and Unicode License v3 custody requirements. The proprietary parent
`src/values/text/mod.rs` is integration reference only and is not copied.
