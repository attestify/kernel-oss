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

The public behavior and private ownership boundary are described in
[`docs/nfc-text.md`](docs/nfc-text.md) and [`docs/url.md`](docs/url.md).

Unicode 17.0 URL/IDNA behavioral fixtures under `tests/data/url/unicode/` and
table-generation inputs under `tools/url-data/data/unicode/` are retained under
this same License v3 custody for the private dependency-free URL-domain and
IDNA implementation. The pinned WPT URL fixtures and their license are retained
under `tests/data/url/` for conformance qualification; the former external
`url` crate is not packaged or used by Kernel OSS 0.3.0.

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

## ulid-rs

The Kernel OSS ULID value and Crockford Base32 implementation contain code
adapted from `dylanhart/ulid-rs`, commit
`482338a638335ee8632cde85a79fc722fda5113e`.

Copyright (c) 2017 Dylan Hart.

The selected license is MIT; its complete text is retained in
[`third-party/licenses/ulid-rs-MIT.txt`](third-party/licenses/ulid-rs-MIT.txt).
Attestify modified the implementation for its bounded Kernel identity value.
`ulid-rs` is not a Kernel OSS dependency, and its complete crate API,
generators, optional integrations, dependencies, and ABI are not adopted.
