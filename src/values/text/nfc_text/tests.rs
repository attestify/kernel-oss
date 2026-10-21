//! Public `NfcText` value-object verification.
//!
//! Bounded unit: `kernel_oss::values::text::NfcText`, including its public
//! `new`, `is_nfc`, and `Value<ValueType = str>` interfaces. Logical paths:
//! canonical construction, equality/hash, idempotence, whole-string handling,
//! preservation, query outcomes, redacted diagnostics, and intentional public
//! trait exclusions. Drive path, metadata, authorization, persistence,
//! transport, and RegularFile-content behavior are intentionally out of scope.
//!
//! The permanent behavior contract and coverage floor are defined in
//! `docs/values/text/README.md` and `docs/testing.md`.

use super::NfcText;
use crate::values::Value;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_of(text: &NfcText) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

/// Verifies canonical equivalents construct the same borrowed NFC value.
#[test]
fn canonical_equivalents_construct_one_value_success() {
    let composed = NfcText::new("é");
    let decomposed = NfcText::new("e\u{0301}");

    assert_eq!(composed.value(), "é");
    assert_eq!(composed, decomposed);
}

/// Verifies complete-string construction is idempotent after a boundary join.
#[test]
fn complete_string_normalization_is_idempotent_success() {
    let first = NfcText::new("prefix A\u{030A} suffix");
    let second = NfcText::new(first.value());

    assert_eq!(first, second);
    assert_eq!(second.value(), "prefix Å suffix");
}

/// Verifies separately NFC values can require normalization after concatenation.
#[test]
fn construction_normalizes_a_joined_nfc_boundary_success() {
    let starter = NfcText::new("A");
    let combining_mark = NfcText::new("\u{030A}");
    let joined = format!("{}{}", starter.value(), combining_mark.value());

    assert!(!NfcText::is_nfc(&joined));
    assert_eq!(NfcText::new(&joined).value(), "Å");
}

/// Verifies NFC preserves compatibility characters and consumer-owned text.
#[test]
fn canonical_only_normalization_preserves_compatibility_and_controls_success() {
    let input = "\0 / . .. \u{2026}\u{200B}\u{202E}  Mixed Case  ";
    let text = NfcText::new(input);

    assert_eq!(text.value(), input);
}

/// Verifies valid maximum scalar text is retained without an invented error.
#[test]
fn maximum_scalar_and_empty_input_are_retained_success() {
    assert_eq!(NfcText::new("").value(), "");
    assert_eq!(NfcText::new("\u{10FFFF}").value(), "\u{10FFFF}");
}

/// Verifies equal normalized values have equal hash output.
#[test]
fn normalized_equality_and_hash_are_aligned_success() {
    let composed = NfcText::new("Å");
    let decomposed = NfcText::new("A\u{030A}");

    assert_eq!(composed, decomposed);
    assert_eq!(hash_of(&composed), hash_of(&decomposed));
}

/// Verifies canonically equivalent values deduplicate in a hash collection.
#[test]
fn canonically_equivalent_values_deduplicate_in_a_hash_set_success() {
    let mut values = HashSet::new();
    values.insert(NfcText::new("Å"));
    values.insert(NfcText::new("A\u{030A}"));

    assert_eq!(values.len(), 1);
    assert!(values.contains(&NfcText::new("Å")));
}

/// Verifies diagnostics use the exact redacted contract and disclose no input.
#[test]
fn debug_output_is_exactly_redacted_success() {
    let sensitive = "person@example.test\u{0301}";
    let text = NfcText::new(sensitive);
    let diagnostic = format!("{text:?}");

    assert_eq!(diagnostic, "NfcText([REDACTED])");
    assert!(!diagnostic.contains(sensitive));
    assert!(!diagnostic.contains(text.value()));
}

/// Verifies the public query distinguishes already-normalized and decomposed text.
#[test]
fn is_nfc_uses_the_same_normalization_result_success() {
    assert!(NfcText::is_nfc("Å"));
    assert!(!NfcText::is_nfc("A\u{030A}"));
}
