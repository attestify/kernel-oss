//! Private application/x-www-form-urlencoded parsing.
#![cfg_attr(
    not(test),
    expect(dead_code, reason = "P3 private primitives are wired by P4 parsing.")
)]

#[cfg(test)]
mod tests;

use super::percent_encoding::{decode_bytes, decode_utf8_replacement};

/// Parses ordered form pairs with one percent-decoding pass and UTF-8 replacement.
pub(super) fn parse(input: &str) -> Vec<(String, String)> {
    if input.is_empty() {
        return Vec::new();
    }
    input
        .split('&')
        .filter(|sequence| !sequence.is_empty())
        .map(parse_sequence)
        .collect()
}

fn parse_sequence(sequence: &str) -> (String, String) {
    let (name, value) = sequence.split_once('=').unwrap_or((sequence, ""));
    (decode_form(name), decode_form(value))
}

fn decode_form(value: &str) -> String {
    let plus_as_space = value.replace('+', " ");
    decode_utf8_replacement(&decode_bytes(&plus_as_space))
}
