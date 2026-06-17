//! Tests for `Block`, covering construction, sanitization, and comparison behavior.
//!
//! Bounded unit under test: `Block`.
//! Public interfaces verified: `default`, `new`, `from_string`, `len`, `is_empty`, and `value`.
//! Logical paths covered: default construction, string construction, sanitization, length,
//! emptiness, and equality comparisons.
//! Requirement validation points: standards-aligned multi-line text block behavior.

use super::Block;

#[test]
/// Requirement validation: verifies the default block is empty.
fn default_success() {
    assert_eq!(Block::default(), "");
}

#[test]
/// Requirement validation: verifies string construction preserves block content.
fn from_str_success() {
    assert_eq!(
        Block::new("This is a new block of text."),
        "This is a new block of text."
    );
}

#[test]
/// Requirement validation: verifies string ownership construction preserves block content.
fn from_string_success() {
    assert_eq!(
        Block::from_string(String::from("This is a new block of text.")),
        "This is a new block of text."
    );
}

#[test]
/// Requirement validation: verifies block construction sanitizes surrounding whitespace.
fn sanitize_from_str_success() {
    assert_eq!(
        Block::new("  Unsanitary \nText block\r\t  "),
        "Unsanitary \nText block"
    );
}

#[test]
/// Requirement validation: verifies owned-string block construction sanitizes surrounding whitespace.
fn sanitize_from_string_success() {
    assert_eq!(
        Block::from_string(String::from("  Unsanitary \nText block\r\t  ")),
        String::from("Unsanitary \nText block")
    );
}

#[test]
/// Requirement validation: verifies block length reflects the sanitized contents.
fn length_success() {
    assert_eq!(
        Block::from_string("\rThis is a new \nblock of \ttext").len(),
        29
    );
}

#[test]
/// Requirement validation: verifies empty and non-empty block detection works.
fn is_empty_success() {
    assert!(Block::default().is_empty());
    assert!(!Block::new("hello").is_empty());
}

#[test]
/// Requirement validation: verifies the block value accessor returns the underlying text.
fn value_success() {
    assert_eq!(Block::new("hello line").value(), "hello line")
}

#[test]
/// Requirement validation: verifies block comparison against owned strings works.
fn compare_to_string_success() {
    assert_eq!(
        Block::from_string("This is a new block of text"),
        String::from("This is a new block of text")
    )
}

#[test]
/// Requirement validation: verifies string comparison against blocks works.
fn compare_from_string_success() {
    assert_eq!(
        String::from("This is a new block of text"),
        Block::from_string("This is a new block of text")
    )
}

#[test]
/// Requirement validation: verifies block comparison against string slices works.
fn compare_to_str_success() {
    assert_eq!(
        Block::from_string("This is a new block of text"),
        "This is a new block of text"
    )
}

#[test]
/// Requirement validation: verifies string-slice comparison against blocks works.
fn compare_from_str_success() {
    assert_eq!(
        "This is a new block of text",
        Block::from_string("This is a new block of text")
    )
}

#[test]
/// Requirement validation: verifies identical blocks compare equal.
fn compare_equal_blocks_success() {
    assert_eq!(
        Block::from_string("\rThis is a new \nblock of \ttext"),
        Block::from_string("\rThis is a new \nblock of \ttext")
    )
}

#[test]
/// Requirement validation: verifies distinct blocks compare not equal.
fn compare_different_blocks_success() {
    assert_ne!(
        Block::from_string("\rThis is the first \nblock of \ttext"),
        Block::from_string("\rThis is the second \nblock of \ttext")
    )
}
