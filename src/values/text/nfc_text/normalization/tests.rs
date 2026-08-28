//! Private Unicode 17.0 NFC engine verification.
//!
//! Bounded unit: the private `normalize_nfc` and `is_nfc` engine beneath
//! `NfcText`. Logical paths: empty/ASCII input, recursive decomposition,
//! leading non-starters, combining-class ordering and blocking, composition
//! exclusions, Hangul boundaries, compatibility preservation, unassigned
//! scalars, conformance parsing, and adversarial combining sequences. Public
//! object API, Drive rules, persistence, transport, and file content are out
//! of scope.
//!
//! Supplied requirement validation: `AKNL-NFC-P1-REQ-003` through
//! `AKNL-NFC-P1-REQ-008`, `AKNL-NFC-P1-REQ-015`, `AKNL-NFC-P1-REQ-019`,
//! `AKNL-NFC-P1-REQ-020`, `AKNL-NFC-P1-REQ-025`, `AKNL-NFC-P1-REQ-031`,
//! `AKNL-NFC-P1-REQ-034`, `AKNL-NFC-P1-REQ-038`, `AKNL-NFC-P1-REQ-039`, and
//! `AKNL-NFC-P1-REQ-040`.

use super::{is_virama, normalize_nfc};

#[test]
/// Verifies the crate-private virama query reuses canonical combining classes.
fn virama_query_uses_shared_table_success() {
    assert!(is_virama('\u{094D}'));
    assert!(!is_virama('a'));
}

/// Requirement validation: `AKNL-NFC-P1-AC-003` and `AKNL-NFC-P1-AC-007`.
///
/// Verifies ASCII text requires no canonical normalization change.
#[test]
fn ascii_text_is_preserved_success() {
    assert_eq!(normalize_nfc("abc"), "abc");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies canonical singleton decomposition is recomposed to NFC.
#[test]
fn canonical_singleton_is_normalized_success() {
    assert_eq!(normalize_nfc("\u{2126}"), "Ω");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies recursive canonical decomposition reaches its NFC result.
#[test]
fn recursive_canonical_decomposition_is_normalized_success() {
    assert_eq!(normalize_nfc("\u{212B}"), "Å");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies a leading non-starter remains before a following starter.
#[test]
fn leading_non_starter_is_preserved_success() {
    assert_eq!(normalize_nfc("\u{0301}a"), "\u{0301}a");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies equal combining classes retain their original stable order.
#[test]
fn equal_combining_classes_keep_stable_order_success() {
    assert_eq!(normalize_nfc("\u{0300}\u{0301}a"), "\u{0300}\u{0301}a");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies decomposition, canonical ordering, and permitted composition.
#[test]
fn canonical_ordering_and_composition_produce_nfc_success() {
    assert_eq!(normalize_nfc("\u{1E0B}\u{0323}"), "\u{1E0D}\u{0307}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-003`.
///
/// Verifies combining-class blocking prevents an invalid composition.
#[test]
fn blocked_composition_preserves_the_required_order_success() {
    assert_eq!(
        normalize_nfc("a\u{0300}\u{0305}\u{0315}\u{05AE}b"),
        "à\u{05AE}\u{0305}\u{0315}b"
    );
}

/// Requirement validation: `AKNL-NFC-P1-AC-034`.
///
/// Verifies already composed Hangul syllables remain stable.
#[test]
fn composed_hangul_syllables_are_stable_success() {
    assert_eq!(normalize_nfc("\u{D4DB}"), "\u{D4DB}");
    assert_eq!(normalize_nfc("\u{AC1C}"), "\u{AC1C}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-034`.
///
/// Verifies first leading/vowel Jamo compose to the first Hangul syllable.
#[test]
fn first_hangul_leading_and_vowel_jamo_compose_success() {
    assert_eq!(normalize_nfc("\u{1100}\u{1161}"), "\u{AC00}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-034`.
///
/// Verifies the first LV syllable and trailing Jamo compose.
#[test]
fn first_hangul_lv_and_trailing_jamo_compose_success() {
    assert_eq!(normalize_nfc("\u{AC00}\u{11A8}"), "\u{AC01}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-034`.
///
/// Verifies the final leading, vowel, and trailing Jamo reach the final syllable.
#[test]
fn final_hangul_jamo_boundaries_compose_success() {
    assert_eq!(normalize_nfc("\u{1112}\u{1175}\u{11C2}"), "\u{D7A3}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-034`.
///
/// Verifies T_BASE itself cannot compose with an LV syllable.
#[test]
fn hangul_t_base_does_not_compose_success() {
    assert_eq!(normalize_nfc("\u{C8E0}\u{11A7}"), "\u{C8E0}\u{11A7}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-006`.
///
/// Verifies NFC preserves a compatibility character rather than expanding it.
#[test]
fn compatibility_character_is_preserved_success() {
    assert_eq!(normalize_nfc("\u{2026}"), "\u{2026}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-033` and `AKNL-NFC-P1-AC-034`.
///
/// Verifies a full-composition exclusion stays decomposed after NFC.
#[test]
fn full_composition_exclusion_is_not_recomposed_success() {
    assert_eq!(normalize_nfc("\u{0344}"), "\u{0308}\u{0301}");
}

/// Requirement validation: `AKNL-NFC-P1-AC-007` and `AKNL-NFC-P1-AC-039`.
///
/// Verifies stable equal-class order and a long adversarial mark sequence.
#[test]
fn combining_mark_ordering_is_stable_under_long_input_success() {
    let input = format!("a{}", "\u{0315}\u{0300}".repeat(512));
    let normalized = normalize_nfc(&input);

    assert!(normalized.starts_with("à"));
    assert_eq!(normalize_nfc(&normalized), normalized);
}
