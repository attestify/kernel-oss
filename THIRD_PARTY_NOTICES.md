# Third-party notices

## Unicode 17.0 NFC data

Kernel OSS includes the official Unicode 17.0 data and fixture artifacts used
to generate and qualify the private shared NFC tables. They are retained under
the Unicode License v3, whose full text is in
[`third-party/licenses/Unicode-3.0.txt`](third-party/licenses/Unicode-3.0.txt).
The exact approved input hashes are recorded in
[`tools/unicode-nfc/checksums.sha256`](tools/unicode-nfc/checksums.sha256).

The data supports the private normalization engine behind
`kernel_oss::values::text::NfcText`. It does not expose a public Unicode or
normalization API.

Unicode 17.0 URL/IDNA inputs under `test_data/url/unicode/` are retained under
this same License v3 custody for the private dependency-free URL-domain and
IDNA implementation. The pinned WPT URL fixtures and their license are retained
under `test_data/url/` for conformance qualification; the former external `url`
crate is not packaged or used by Kernel OSS 0.3.0.

## unicode-normalization 0.1.25

The private NFC normalizer contains narrowly recorded algorithmic influence
from `unicode-rs/unicode-normalization` 0.1.25, commit
`7ae3eb01d3c30da0d3c319225b306d8315ceb134`.

Copyright 2012-2015 The Rust Project Developers.

The selected license is MIT; its complete text is retained in
[`third-party/licenses/unicode-normalization-MIT.txt`](third-party/licenses/unicode-normalization-MIT.txt).
Attestify modified and relocated the NFC-only engine in 2026 while retaining
the required attribution. The crate is not a Kernel OSS dependency, and its
public traits, iterators, module layout, generated tables, quick-check API,
other normalization forms, and ABI are not adopted.
