//! Exhaustive public-boundary Unicode 17.0 NFC qualification for `NfcText`.
//!
//! Bounded unit: `kernel_oss::values::text::NfcText` public construction and
//! query interfaces, with `kernel_oss::values::Value` for borrowed output. Logical
//! paths: all official Unicode 17.0 NFC fixture rows, strict fixture census,
//! Part 1 single-scalar oracle construction, and every valid Rust Unicode
//! scalar, including malformed-fixture rejection paths. Private normalization
//! helpers, generated tables, production source, Drive behavior, persistence,
//! transport, and product validation are out of scope. Supplied requirement
//! validation: `AKNL-NFC-Q2-REQ-003` through `AKNL-NFC-Q2-REQ-013`,
//! `AKNL-NFC-Q2-REQ-019` through `AKNL-NFC-Q2-REQ-022`.

use std::collections::HashMap;
use std::sync::OnceLock;

use kernel_oss::values::Value;
use kernel_oss::values::text::NfcText;

const NORMALIZATION_TEST: &str = include_str!("data/nfc/unicode/NormalizationTest-17.0.0.txt");
const EXPECTED_PART_COUNTS: [usize; 6] = [45, 17_086, 1_936, 194, 735, 38];
const EXPECTED_RECORDS: usize = 20_034;
const EXPECTED_VALID_SCALARS: usize = 1_112_064;
const EXPECTED_PART_ONE_SCALARS: usize = 17_086;
const EXPECTED_IDENTITY_SCALARS: usize = 1_094_978;

#[derive(Clone)]
struct FixtureRow {
    line_number: usize,
    part: usize,
    columns: [String; 5],
}

fn parse_scalar_column(
    column: &str,
    line_number: usize,
    column_number: usize,
) -> Result<String, String> {
    if column.trim().is_empty() {
        return Err(format!(
            "fixture line {line_number} column {column_number}: empty scalar column"
        ));
    }

    let mut parsed = String::new();
    for token in column.split_whitespace() {
        let scalar = u32::from_str_radix(token, 16).map_err(|_| {
            format!(
                "fixture line {line_number} column {column_number}: invalid hexadecimal scalar {token}"
            )
        })?;
        let character = char::from_u32(scalar).ok_or_else(|| {
            format!(
                "fixture line {line_number} column {column_number}: invalid Unicode scalar {token}"
            )
        })?;
        parsed.push(character);
    }
    Ok(parsed)
}

fn parse_fixture_row(line: &str, line_number: usize, part: usize) -> Result<FixtureRow, String> {
    let body = line
        .split_once('#')
        .map_or(line, |(prefix, _)| prefix)
        .trim();
    let mut fields: Vec<&str> = body.split(';').collect();
    if fields.len() == 6 && fields[5].trim().is_empty() {
        fields.pop();
    }
    if fields.len() != 5 {
        return Err(format!(
            "fixture line {line_number}: expected five columns, found {}",
            fields.len()
        ));
    }

    let mut columns = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for (index, field) in fields.into_iter().enumerate() {
        columns[index] = parse_scalar_column(field.trim(), line_number, index + 1)?;
    }

    Ok(FixtureRow {
        line_number,
        part,
        columns,
    })
}

fn parse_part_marker(line: &str, line_number: usize) -> Result<usize, String> {
    let marker = line
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("fixture line {line_number}: empty part marker"))?;
    let number = marker
        .strip_prefix("@Part")
        .ok_or_else(|| format!("fixture line {line_number}: unknown part marker {marker}"))?
        .parse::<usize>()
        .map_err(|_| format!("fixture line {line_number}: unknown part marker {marker}"))?;
    if number >= EXPECTED_PART_COUNTS.len() {
        return Err(format!(
            "fixture line {line_number}: unknown part marker {marker}"
        ));
    }
    Ok(number)
}

fn parse_fixture_input(input: &str) -> Result<Vec<FixtureRow>, String> {
    let mut rows = Vec::new();
    let mut current_part = None;
    let mut seen_parts = [false; 6];

    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('@') {
            let part = parse_part_marker(trimmed, line_number)?;
            if seen_parts[part] {
                return Err(format!(
                    "fixture line {line_number}: duplicate Part {part} marker"
                ));
            }
            seen_parts[part] = true;
            current_part = Some(part);
            continue;
        }
        let part = current_part.ok_or_else(|| {
            format!("fixture line {line_number}: row appears before a known part marker")
        })?;
        rows.push(parse_fixture_row(line, line_number, part)?);
    }

    if seen_parts != [true; 6] {
        return Err("fixture does not contain exactly Parts 0 through 5".to_owned());
    }
    Ok(rows)
}

fn parse_fixture() -> Result<Vec<FixtureRow>, String> {
    parse_fixture_input(NORMALIZATION_TEST)
}

fn fixture_rows() -> &'static [FixtureRow] {
    static ROWS: OnceLock<Vec<FixtureRow>> = OnceLock::new();
    ROWS.get_or_init(|| parse_fixture().expect("committed Unicode fixture must parse"))
}

fn build_part_one_oracle(rows: &[FixtureRow]) -> Result<HashMap<u32, String>, String> {
    let mut oracle = HashMap::new();
    for row in rows.iter().filter(|row| row.part == 1) {
        let mut characters = row.columns[0].chars();
        let scalar = characters.next().ok_or_else(|| {
            format!(
                "fixture line {}: Part 1 source has no scalar",
                row.line_number
            )
        })?;
        if characters.next().is_some() {
            return Err(format!(
                "fixture line {}: Part 1 source has more than one scalar",
                row.line_number
            ));
        }
        if oracle
            .insert(scalar as u32, row.columns[1].clone())
            .is_some()
        {
            return Err(format!(
                "fixture line {}: duplicate Part 1 scalar",
                row.line_number
            ));
        }
    }
    Ok(oracle)
}

fn part_one_oracle() -> &'static HashMap<u32, String> {
    static ORACLE: OnceLock<HashMap<u32, String>> = OnceLock::new();
    ORACLE.get_or_init(|| {
        build_part_one_oracle(fixture_rows()).expect("Part 1 oracle fixture setup must succeed")
    })
}

fn assert_fixture_error<T>(result: Result<T, String>, expected: &str) {
    assert!(
        result.is_err(),
        "expected fixture error containing {expected:?}"
    );
    if let Err(error) = result {
        assert!(
            error.contains(expected),
            "expected fixture error containing {expected:?}, found {error:?}"
        );
    }
}

fn synthetic_part_one_row(line_number: usize, source: &str) -> FixtureRow {
    FixtureRow {
        line_number,
        part: 1,
        columns: [
            source.to_owned(),
            "A".to_owned(),
            "A".to_owned(),
            "A".to_owned(),
            "A".to_owned(),
        ],
    }
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-004`, `AKNL-NFC-Q2-REQ-008`,
/// and `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies empty, non-hexadecimal, and non-scalar fixture columns report
/// their exact source context rather than being accepted or normalized.
#[test]
fn malformed_scalar_columns_return_contextual_errors_error() {
    assert_fixture_error(
        parse_scalar_column("", 7, 3),
        "fixture line 7 column 3: empty scalar column",
    );
    assert_fixture_error(
        parse_scalar_column("not-hex", 8, 2),
        "fixture line 8 column 2: invalid hexadecimal scalar not-hex",
    );
    assert_fixture_error(
        parse_scalar_column("110000", 9, 1),
        "fixture line 9 column 1: invalid Unicode scalar 110000",
    );
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-004`, `AKNL-NFC-Q2-REQ-007`,
/// and `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies structural fixture failures reject malformed rows, invalid or
/// duplicate part markers, and rows before a known part marker.
#[test]
fn malformed_fixture_structure_returns_contextual_errors_error() {
    assert_fixture_error(
        parse_fixture_input("@Part0\n0041;0041;0041;0041\n"),
        "fixture line 2: expected five columns, found 4",
    );
    assert_fixture_error(
        parse_fixture_input("@Part6\n"),
        "fixture line 1: unknown part marker @Part6",
    );
    assert_fixture_error(
        parse_fixture_input("@Unexpected\n"),
        "fixture line 1: unknown part marker @Unexpected",
    );
    assert_fixture_error(
        parse_fixture_input("@Part0\n@Part0\n"),
        "fixture line 2: duplicate Part 0 marker",
    );
    assert_fixture_error(
        parse_fixture_input("0041;0041;0041;0041;0041\n"),
        "fixture line 1: row appears before a known part marker",
    );
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-007`, `AKNL-NFC-Q2-REQ-008`,
/// and `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies the Part 1 scalar oracle rejects empty, multi-scalar, and
/// duplicate sources while retaining its bounded synthetic fixture input.
#[test]
fn invalid_part_one_oracle_sources_return_contextual_errors_error() {
    assert_fixture_error(
        build_part_one_oracle(&[synthetic_part_one_row(11, "")]),
        "fixture line 11: Part 1 source has no scalar",
    );
    assert_fixture_error(
        build_part_one_oracle(&[synthetic_part_one_row(12, "AB")]),
        "fixture line 12: Part 1 source has more than one scalar",
    );
    assert_fixture_error(
        build_part_one_oracle(&[
            synthetic_part_one_row(13, "A"),
            synthetic_part_one_row(14, "A"),
        ]),
        "fixture line 14: duplicate Part 1 scalar",
    );
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-004`, `AKNL-NFC-Q2-REQ-007`,
/// and `AKNL-NFC-Q2-REQ-008`.
///
/// Verifies the exact official fixture sections, total rows, and unique
/// single-scalar Part 1 oracle inputs.
#[test]
fn official_unicode_17_fixture_census_success() {
    let rows = fixture_rows();
    let mut part_counts = [0; 6];
    for row in rows {
        part_counts[row.part] += 1;
    }

    assert_eq!(
        part_counts, EXPECTED_PART_COUNTS,
        "official fixture part census"
    );
    assert_eq!(rows.len(), EXPECTED_RECORDS, "official fixture row census");
    assert_eq!(
        part_one_oracle().len(),
        EXPECTED_PART_ONE_SCALARS,
        "Part 1 scalar oracle census"
    );
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-003`, `AKNL-NFC-Q2-REQ-004`,
/// `AKNL-NFC-Q2-REQ-005`, and `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies all five official NFC construction invariants through public APIs.
#[test]
fn official_unicode_17_rows_construct_expected_public_values_success() {
    for row in fixture_rows() {
        let [c1, c2, c3, c4, c5] = &row.columns;
        assert_eq!(
            NfcText::new(c1).value(),
            c2.as_str(),
            "fixture line {} c1",
            row.line_number
        );
        assert_eq!(
            NfcText::new(c2).value(),
            c2.as_str(),
            "fixture line {} c2",
            row.line_number
        );
        assert_eq!(
            NfcText::new(c3).value(),
            c2.as_str(),
            "fixture line {} c3",
            row.line_number
        );
        assert_eq!(
            NfcText::new(c4).value(),
            c4.as_str(),
            "fixture line {} c4",
            row.line_number
        );
        assert_eq!(
            NfcText::new(c5).value(),
            c4.as_str(),
            "fixture line {} c5",
            row.line_number
        );
    }
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-003`, `AKNL-NFC-Q2-REQ-004`,
/// `AKNL-NFC-Q2-REQ-006`, and `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies public query results for every official fixture column.
#[test]
fn official_unicode_17_rows_report_expected_public_nfc_state_success() {
    for row in fixture_rows() {
        for (index, column) in row.columns.iter().enumerate() {
            let expected = if index < 3 {
                row.columns[1].as_str()
            } else {
                row.columns[3].as_str()
            };
            assert_eq!(
                NfcText::is_nfc(column),
                column == expected,
                "fixture line {} c{}",
                row.line_number,
                index + 1
            );
        }
    }
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-003`, `AKNL-NFC-Q2-REQ-008`,
/// `AKNL-NFC-Q2-REQ-009`, `AKNL-NFC-Q2-REQ-010`, `AKNL-NFC-Q2-REQ-011`, and
/// `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies public construction for every valid Rust Unicode scalar while
/// streaming inputs rather than retaining the scalar space.
#[test]
fn every_valid_unicode_scalar_constructs_expected_public_value_success() {
    let oracle = part_one_oracle();
    let mut valid_scalars = 0;
    let mut explicit_results = 0;
    let mut identity_results = 0;

    for scalar in 0..=0x10FFFF {
        let Some(character) = char::from_u32(scalar) else {
            continue;
        };
        let input = character.to_string();
        let expected = match oracle.get(&scalar) {
            Some(value) => {
                explicit_results += 1;
                value.as_str()
            }
            None => {
                identity_results += 1;
                input.as_str()
            }
        };
        assert_eq!(
            NfcText::new(&input).value(),
            expected,
            "scalar U+{scalar:04X} construction"
        );
        valid_scalars += 1;
    }

    assert_eq!(valid_scalars, EXPECTED_VALID_SCALARS, "valid scalar census");
    assert_eq!(
        explicit_results, EXPECTED_PART_ONE_SCALARS,
        "Part 1 scalar census"
    );
    assert_eq!(
        identity_results, EXPECTED_IDENTITY_SCALARS,
        "identity scalar census"
    );
}

/// Requirement validation: `AKNL-NFC-Q2-REQ-003`, `AKNL-NFC-Q2-REQ-008`,
/// `AKNL-NFC-Q2-REQ-009`, `AKNL-NFC-Q2-REQ-010`, `AKNL-NFC-Q2-REQ-012`, and
/// `AKNL-NFC-Q2-REQ-013`.
///
/// Verifies public query results for every valid Rust Unicode scalar while
/// streaming inputs rather than retaining the scalar space.
#[test]
fn every_valid_unicode_scalar_reports_expected_public_nfc_state_success() {
    let oracle = part_one_oracle();
    let mut valid_scalars = 0;
    let mut explicit_results = 0;
    let mut identity_results = 0;

    for scalar in 0..=0x10FFFF {
        let Some(character) = char::from_u32(scalar) else {
            continue;
        };
        let input = character.to_string();
        let expected = match oracle.get(&scalar) {
            Some(value) => {
                explicit_results += 1;
                value.as_str()
            }
            None => {
                identity_results += 1;
                input.as_str()
            }
        };
        assert_eq!(
            NfcText::is_nfc(&input),
            input == expected,
            "scalar U+{scalar:04X} query"
        );
        valid_scalars += 1;
    }

    assert_eq!(valid_scalars, EXPECTED_VALID_SCALARS, "valid scalar census");
    assert_eq!(
        explicit_results, EXPECTED_PART_ONE_SCALARS,
        "Part 1 scalar census"
    );
    assert_eq!(
        identity_results, EXPECTED_IDENTITY_SCALARS,
        "identity scalar census"
    );
}
