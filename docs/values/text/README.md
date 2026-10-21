# Text values: NFC text

[Documentation index](../../README.md)

Use `NfcText` when canonically equivalent Unicode spellings should compare as
the same text. Construction is infallible and accepts any Rust string; it does
not validate application-specific text rules.

## Public contract

`kernel_oss::values::text::NfcText` is an immutable Unicode 17 NFC value. It
normalizes input during construction and implements
`Value<ValueType = str>` for borrowed canonical access.

```rust
use kernel_oss::values::Value;
use kernel_oss::values::text::NfcText;

let text = NfcText::new("e\u{301}");

assert_eq!(text.value(), "é");
assert!(NfcText::is_nfc(text.value()));
assert!(!NfcText::is_nfc("e\u{301}"));
```

Equality and hashing use normalized value semantics. `NfcText` is `Clone`,
`Send`, and `Sync`. `Debug` is exactly `NfcText([REDACTED])`; the type has no
general `Display` representation.

## Design and verification

See [Design and maintenance](design.md) for private architecture, source and data
custody, and verification commands. These details do not change the public API.
