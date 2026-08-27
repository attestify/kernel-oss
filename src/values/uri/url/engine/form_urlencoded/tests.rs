//! Tests for private form query parsing.

use super::parse;

#[test]
/// Verifies order, duplicates, empty names and first-equals splitting.
fn ordered_pairs_success() {
    assert_eq!(
        parse("a=1&a=2&=x&empty=&a=b=c"),
        vec![
            ("a".to_owned(), "1".to_owned()),
            ("a".to_owned(), "2".to_owned()),
            ("".to_owned(), "x".to_owned()),
            ("empty".to_owned(), "".to_owned()),
            ("a".to_owned(), "b=c".to_owned()),
        ]
    );
}

#[test]
/// Verifies plus and percent decoding happen once with replacement.
fn decoding_success() {
    assert_eq!(
        parse("a+b=c%2Bd&x=%25%32%36&bad=%FF"),
        vec![
            ("a b".to_owned(), "c+d".to_owned()),
            ("x".to_owned(), "%26".to_owned()),
            ("bad".to_owned(), "�".to_owned()),
        ]
    );
}

#[test]
/// Verifies repeated delimiters retain empty sequences.
fn repeated_delimiters_success() {
    assert_eq!(parse("&&"), vec![("".to_owned(), "".to_owned()); 3]);
}

#[test]
/// Verifies an empty query has no form sequences.
fn empty_input_success() {
    assert!(parse("").is_empty());
}
