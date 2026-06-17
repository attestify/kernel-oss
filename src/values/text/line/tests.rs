//! Tests for `Line`, covering construction, sanitization, and comparison behavior.
//!
//! Bounded unit under test: `Line`.
//! Public interfaces verified: `default`, `new`, `from_string`, `len`, `is_empty`, and `value`.
//! Logical paths covered: default construction, string construction, sanitization, length,
//! emptiness, and equality comparisons.
//! Requirement validation points: standards-aligned single-line text value behavior.

use super::Line;

#[test]
/// Requirement validation: verifies the default line is empty.
fn default_success() {
    assert_eq!(Line::default(), "");
}

#[test]
/// Requirement validation: verifies string construction preserves line content.
fn from_str_success() {
    assert_eq!(
        Line::new("This is a new line of text"),
        ("This is a new line of text")
    );
}

#[test]
/// Requirement validation: verifies owned-string construction preserves line content.
fn from_string_success() {
    assert_eq!(
        Line::from_string(String::from("This is a new line of text")),
        String::from("This is a new line of text")
    );
}

#[test]
/// Requirement validation: verifies owned-string line construction sanitizes whitespace.
fn sanitize_from_string_success() {
    assert_eq!(
        Line::from_string(String::from("  Unsanitary \nText Line\r\t  ")),
        String::from("Unsanitary Text Line")
    );
}

#[test]
/// Requirement validation: verifies string-slice line construction sanitizes whitespace.
fn sanitize_from_str_success() {
    assert_eq!(
        Line::new("  Unsanitary \nText Line\r\t  "),
        ("Unsanitary Text Line")
    );
}

#[test]
/// Requirement validation: verifies line length reflects the sanitized contents.
fn length_success() {
    assert_eq!(Line::from_string("This is a new line of text").len(), 26);
}

#[test]
/// Requirement validation: verifies empty and non-empty line detection works.
fn is_empty_success() {
    assert!(Line::default().is_empty());
    assert!(!Line::new("hello").is_empty());
}

#[test]
/// Requirement validation: verifies the line value accessor returns the underlying text.
fn value_success() {
    assert_eq!(Line::new("hello line").value(), "hello line")
}

#[test]
/// Requirement validation: verifies line comparison against owned strings works.
fn compare_to_string_success() {
    assert_eq!(
        Line::from_string("This is a new line of text"),
        String::from("This is a new line of text")
    )
}

#[test]
/// Requirement validation: verifies string comparison against lines works.
fn compare_from_string_success() {
    assert_eq!(
        String::from("This is a new line of text"),
        Line::from_string("This is a new line of text")
    )
}

#[test]
/// Requirement validation: verifies line comparison against string slices works.
fn compare_to_str_success() {
    assert_eq!(
        Line::from_string("This is a new line of text"),
        "This is a new line of text"
    )
}

#[test]
/// Requirement validation: verifies string-slice comparison against lines works.
fn compare_from_str_success() {
    assert_eq!(
        "This is a new line of text",
        Line::from_string("This is a new line of text")
    )
}

#[test]
/// Requirement validation: verifies identical lines compare equal.
fn compare_equal_lines_success() {
    assert_eq!(
        Line::from_string("This is a new line of text"),
        Line::from_string("This is a new line of text")
    )
}

#[test]
/// Requirement validation: verifies distinct lines compare not equal.
fn compare_different_lines_success() {
    assert_ne!(
        Line::from_string("First line of text"),
        Line::from_string("Second line of text")
    )
}
