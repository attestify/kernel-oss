//! Exhaustive private NFC engine conformance tests.
//!
//! Bounded unit under test: the crate-private `normalize_nfc` and `is_nfc`
//! engine. These tests retain private-engine qualification that cannot replace
//! the public `NfcText` integration suite.

use super::{is_nfc, normalize_nfc};

const NORMALIZATION_TEST: &str =
    include_str!("../../../../../tests/data/nfc/unicode/NormalizationTest-17.0.0.txt");

fn parse_scalars(column: &str) -> Result<String, String> {
    let mut output = String::new();
    for scalar in column.split_whitespace() {
        let value = u32::from_str_radix(scalar, 16)
            .map_err(|_| format!("invalid hexadecimal scalar: {scalar}"))?;
        let character =
            char::from_u32(value).ok_or_else(|| format!("invalid Unicode scalar: {scalar}"))?;
        output.push(character);
    }
    Ok(output)
}

fn parse_record(line: &str) -> Result<[String; 5], String> {
    let body = line
        .split_once('#')
        .map_or(line, |(prefix, _)| prefix)
        .trim();
    let mut fields: Vec<&str> = body.split(';').collect();
    if fields.len() == 6 && fields[5].trim().is_empty() {
        fields.pop();
    }
    if fields.len() != 5 {
        return Err(format!("expected five columns, found {}", fields.len()));
    }
    let mut columns = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for (index, field) in fields.into_iter().enumerate() {
        columns[index] = parse_scalars(field.trim())?;
    }
    Ok(columns)
}

/// Verifies every official Unicode 17.0 five-column NFC invariant.
#[test]
fn unicode_17_nfc_conformance_fixture_success() {
    let mut records = 0;
    for (line_number, line) in NORMALIZATION_TEST.lines().enumerate() {
        let body = line
            .split_once('#')
            .map_or(line, |(prefix, _)| prefix)
            .trim();
        if body.is_empty() || body.starts_with('@') {
            continue;
        }
        let record = match parse_record(body) {
            Ok(record) => record,
            Err(error) => {
                assert!(
                    error.is_empty(),
                    "malformed Unicode fixture line {}: {error}",
                    line_number + 1
                );
                continue;
            }
        };
        let [c1, c2, c3, c4, c5] = record;
        assert_eq!(normalize_nfc(&c1), c2, "line {} c1", line_number + 1);
        assert_eq!(normalize_nfc(&c2), c2, "line {} c2", line_number + 1);
        assert_eq!(normalize_nfc(&c3), c2, "line {} c3", line_number + 1);
        assert_eq!(normalize_nfc(&c4), c4, "line {} c4", line_number + 1);
        assert_eq!(normalize_nfc(&c5), c4, "line {} c5", line_number + 1);
        assert!(is_nfc(&c2), "line {} c2 query", line_number + 1);
        assert!(is_nfc(&c4), "line {} c4 query", line_number + 1);
        records += 1;
    }
    assert_eq!(records, 20_034);
}

/// Verifies malformed five-column fixture records are rejected rather than skipped.
#[test]
fn malformed_conformance_record_is_rejected_error() {
    assert!(parse_record("0061; 0061; 0061").is_err());
    assert!(parse_record("not-a-scalar; 0061; 0061; 0061; 0061").is_err());
}
