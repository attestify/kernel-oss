//! Unicode 17.0 NFC text owned by Kernel OSS.

#[cfg(test)]
mod tests;

pub(crate) mod normalization;

use crate::values::Value;
use core::fmt;
use core::hash::{Hash, Hasher};

/// One immutable Unicode 17.0 Normalization Form C (NFC) text value.
///
/// Construction converts the complete input to NFC, so canonically equivalent
/// input produces equal values. NFC is canonical normalization only: it does
/// not trim, fold case, apply compatibility normalization, validate product
/// semantics, or alter stored file content. [`Value::value`] intentionally
/// exposes the normalized text; [`fmt::Debug`] always redacts it.
///
/// # Examples
///
/// Canonically equivalent composed and decomposed input has one value:
///
/// ```
/// use kernel_oss::values::text::NfcText;
/// use kernel_oss::values::Value;
///
/// let composed = NfcText::new("é");
/// let decomposed = NfcText::new("e\u{0301}");
///
/// assert_eq!(composed, decomposed);
/// assert_eq!(decomposed.value(), "é");
/// ```
///
/// Raw-field construction is not part of the public contract:
///
/// ```compile_fail
/// use kernel_oss::values::text::NfcText;
///
/// let _ = NfcText {
///     value: String::from("text"),
/// };
/// ```
///
/// `NfcText` has no general display representation:
///
/// ```compile_fail
/// use kernel_oss::values::text::NfcText;
///
/// let text = NfcText::new("sensitive text");
/// let _ = format!("{}", text);
/// ```
///
/// The shared normalization engine is not public API:
///
/// ```compile_fail
/// use kernel_oss::values::text::nfc_text::normalization;
///
/// let _ = normalization::normalize_nfc("text");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct NfcText {
    value: String,
}

impl NfcText {
    /// Converts one complete Unicode string to Unicode 17.0 NFC.
    ///
    /// The operation is infallible for Rust `&str` input and retains only the
    /// owned normalized result. It does not apply product validation, case
    /// conversion, whitespace transformation, compatibility normalization, or
    /// file-content processing.
    ///
    /// # Arguments
    ///
    /// * `text` - Any valid Rust Unicode string slice.
    ///
    /// # Returns
    ///
    /// An immutable value holding the Unicode 17.0 NFC form of `text`.
    ///
    /// # Examples
    ///
    /// Constructing an NFC value again is idempotent:
    ///
    /// ```
    /// use kernel_oss::values::text::NfcText;
    /// use kernel_oss::values::Value;
    ///
    /// let first = NfcText::new("A\u{030A}");
    /// let second = NfcText::new(first.value());
    ///
    /// assert_eq!(first, second);
    /// assert_eq!(second.value(), "Å");
    /// ```
    pub fn new(text: &str) -> Self {
        Self {
            value: normalization::normalize_nfc(text),
        }
    }

    /// Returns whether `text` is already Unicode 17.0 NFC.
    ///
    /// This query uses the same private NFC engine as construction and compares
    /// its resulting complete string with the exact input. It does not mutate
    /// or otherwise transform the caller's text.
    ///
    /// # Arguments
    ///
    /// * `text` - Any valid Rust Unicode string slice to inspect.
    ///
    /// # Returns
    ///
    /// `true` exactly when `text` is already in Unicode 17.0 NFC.
    ///
    /// # Examples
    ///
    /// ```
    /// use kernel_oss::values::text::NfcText;
    ///
    /// assert!(NfcText::is_nfc("é"));
    /// assert!(!NfcText::is_nfc("e\u{0301}"));
    /// ```
    pub fn is_nfc(text: &str) -> bool {
        normalization::is_nfc(text)
    }
}

impl Hash for NfcText {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Value for NfcText {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

impl fmt::Debug for NfcText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("NfcText([REDACTED])")
    }
}
