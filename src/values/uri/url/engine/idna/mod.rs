//! Private Unicode 17 UTS #46, ContextJ, bidi, and Punycode domain processing.
//!
//! This module has no public surface. P3's host dispatcher is the only planned
//! caller. It deliberately shares Kernel OSS's crate-private NFC operation.

#[cfg(test)]
mod tests;

#[cfg(test)]
mod fixture_vectors;

#[cfg(test)]
mod wpt_vectors;

#[cfg(test)]
mod wpt_idna_v2_vectors;

mod punycode;
mod tables;

use crate::values::text::nfc_text::normalization::{is_virama, normalize_nfc};
use tables::{BIDI, CATEGORY, Entry, JOINING, MAPPING, Property, Status};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Invalid,
    Overflow,
    Context,
    Bidi,
}

/// Applies the WHATWG domain parser's nontransitional UTS #46 ToASCII path.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "P2 private IDNA engine is wired by P3 host parsing."
    )
)]
pub(super) fn to_ascii(domain: &str, be_strict: bool) -> Result<String, Error> {
    let mapped = map(domain, be_strict)?;
    if mapped.is_empty() {
        return if be_strict {
            Err(Error::Invalid)
        } else {
            Ok(String::new())
        };
    }
    let normalized = normalize_nfc(&mapped);
    let labels: Vec<&str> = normalized.split('.').collect();
    if labels.iter().any(|label| label.is_empty()) && be_strict {
        return Err(Error::Invalid);
    }
    let bidi_labels: Vec<String> = labels
        .iter()
        .map(|label| {
            label
                .strip_prefix("xn--")
                .and_then(|payload| punycode::decode(payload).ok())
                .unwrap_or_else(|| (*label).to_owned())
        })
        .collect();
    if bidi_labels.iter().any(|label| is_bidi_label(label)) {
        for label in &bidi_labels {
            validate_bidi(label)?;
        }
    }

    let mut ascii = Vec::new();
    ascii
        .try_reserve(labels.len())
        .map_err(|_| Error::Overflow)?;
    for label in labels {
        if label.is_empty() {
            ascii.push(String::new());
            continue;
        }
        validate_label(label, be_strict)?;
        let encoded = if label.is_ascii() {
            label.to_ascii_lowercase()
        } else {
            format!("xn--{}", punycode::encode(label)?)
        };
        validate_a_label(&encoded)?;
        ascii.push(encoded);
    }
    let output = ascii.join(".");
    if be_strict && (output.len() > 253 || ascii.iter().any(|label| label.len() > 63)) {
        return Err(Error::Invalid);
    }
    Ok(output)
}

/// Applies the WHATWG wrapper, including its relaxed all-ASCII compatibility path.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "P2 private IDNA engine is wired by P3 host parsing."
    )
)]
pub(super) fn domain_to_ascii(domain: &str, be_strict: bool) -> Result<String, Error> {
    let strict_result = to_ascii(domain, true);
    if be_strict {
        return finalize_domain(strict_result?);
    }
    let relaxed_result = if domain.is_ascii() {
        Ok(domain.to_ascii_lowercase())
    } else {
        to_ascii(domain, false)
    };
    finalize_domain(relaxed_result?)
}

fn finalize_domain(ascii: String) -> Result<String, Error> {
    if ascii.chars().any(whatwg_forbidden_domain_code_point) {
        return Err(Error::Invalid);
    }
    if ascii.is_empty() {
        return Err(Error::Invalid);
    }
    Ok(ascii)
}

/// Applies ToUnicode sufficient for the P3 private architecture.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "P2 private IDNA engine is wired by P3 host parsing."
    )
)]
pub(super) fn to_unicode(domain: &str, be_strict: bool) -> Result<String, Error> {
    let ascii = to_ascii(domain, be_strict)?;
    let mut labels = Vec::new();
    for label in ascii.split('.') {
        if let Some(payload) = label.strip_prefix("xn--") {
            let decoded = punycode::decode(payload)?;
            if format!("xn--{}", punycode::encode(&decoded)?) != label {
                return Err(Error::Invalid);
            }
            labels.push(decoded);
        } else {
            labels.push(label.to_owned());
        }
    }
    Ok(labels.join("."))
}

fn map(domain: &str, _be_strict: bool) -> Result<String, Error> {
    let mut mapped = String::new();
    for character in domain.chars() {
        let entry = lookup_mapping(u32::from(character)).ok_or(Error::Invalid)?;
        match entry.status {
            Status::Valid | Status::Deviation => mapped.push(character),
            Status::Mapped => mapped.push_str(entry.mapping),
            Status::Ignored => {}
            Status::Disallowed => return Err(Error::Invalid),
        }
    }
    Ok(mapped)
}

fn validate_label(label: &str, be_strict: bool) -> Result<(), Error> {
    let characters: Vec<char> = label.chars().collect();
    if characters.is_empty()
        || general_category(characters[0]) == "Mn"
        || general_category(characters[0]) == "Mc"
        || general_category(characters[0]) == "Me"
    {
        return Err(Error::Invalid);
    }
    if label.starts_with("xn--") && !label.is_ascii() {
        return Err(Error::Invalid);
    }
    if be_strict
        && (label.starts_with('-')
            || label.ends_with('-')
            || (label.as_bytes().get(2..4) == Some(b"--") && !label.starts_with("xn--")))
    {
        return Err(Error::Invalid);
    }
    validate_context_j(&characters)
}

fn validate_a_label(label: &str) -> Result<(), Error> {
    if !label.is_ascii() || label.is_empty() {
        return Err(Error::Invalid);
    }
    if let Some(payload) = label.strip_prefix("xn--") {
        if payload.is_empty() {
            return Err(Error::Invalid);
        }
        let decoded = punycode::decode(payload)?;
        let processed = normalize_nfc(&map(&decoded, false)?);
        if decoded.is_empty()
            || processed != decoded
            || punycode::encode(&processed)? != payload.to_ascii_lowercase()
        {
            return Err(Error::Invalid);
        }
        validate_label(&processed, false)?;
        if is_bidi_label(&processed) {
            validate_bidi(&processed)?;
        }
    }
    Ok(())
}

fn validate_context_j(characters: &[char]) -> Result<(), Error> {
    for (index, character) in characters.iter().enumerate() {
        match character {
            '\u{200D}' => {
                if index == 0 || !is_virama(characters[index - 1]) {
                    return Err(Error::Context);
                }
            }
            '\u{200C}' => {
                if index > 0 && is_virama(characters[index - 1]) {
                    continue;
                }
                let before = previous_non_transparent(characters, index);
                let after = next_non_transparent(characters, index + 1);
                let valid_before = matches!(before.map(joining_type), Some("L" | "D"));
                let valid_after = matches!(after.map(joining_type), Some("R" | "D"));
                if !(valid_before && valid_after) {
                    return Err(Error::Context);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn previous_non_transparent(characters: &[char], index: usize) -> Option<char> {
    characters[..index]
        .iter()
        .rev()
        .copied()
        .find(|character| joining_type(*character) != "T")
}

fn next_non_transparent(characters: &[char], index: usize) -> Option<char> {
    characters[index..]
        .iter()
        .copied()
        .find(|character| joining_type(*character) != "T")
}

fn validate_bidi(label: &str) -> Result<(), Error> {
    if label.is_empty() {
        return Ok(());
    }
    let classes: Vec<&str> = label.chars().map(bidi_class).collect();
    if matches!(classes[0], "R" | "AL") {
        if !classes.iter().all(|class| {
            matches!(
                *class,
                "R" | "AL" | "AN" | "EN" | "ES" | "CS" | "ET" | "ON" | "BN" | "NSM"
            )
        }) {
            return Err(Error::Bidi);
        }
        let final_class = classes
            .iter()
            .rev()
            .find(|class| **class != "NSM")
            .copied()
            .ok_or(Error::Bidi)?;
        if !matches!(final_class, "R" | "AL" | "EN" | "AN")
            || (classes.contains(&"EN") && classes.contains(&"AN"))
        {
            return Err(Error::Bidi);
        }
    } else {
        if classes[0] != "L"
            || !classes.iter().all(|class| {
                matches!(
                    *class,
                    "L" | "EN" | "ES" | "CS" | "ET" | "ON" | "BN" | "NSM"
                )
            })
        {
            return Err(Error::Bidi);
        }
        let final_class = classes
            .iter()
            .rev()
            .find(|class| **class != "NSM")
            .copied()
            .ok_or(Error::Bidi)?;
        if !matches!(final_class, "L" | "EN") {
            return Err(Error::Bidi);
        }
    }
    Ok(())
}

fn is_bidi_label(label: &str) -> bool {
    label
        .chars()
        .any(|character| matches!(bidi_class(character), "R" | "AL" | "AN"))
}

fn forbidden_domain_code_point(character: char) -> bool {
    matches!(character, '\0' | '\t' | '\n' | '\r')
        || matches!(
            character,
            ' ' | '#' | '/' | ':' | '<' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '|'
        )
}

fn whatwg_forbidden_domain_code_point(character: char) -> bool {
    u32::from(character) <= 0x20
        || character == '\u{007F}'
        || character == '%'
        || forbidden_domain_code_point(character)
}

fn joining_type(character: char) -> &'static str {
    lookup(JOINING, u32::from(character)).map_or("U", |entry| entry.value)
}
fn bidi_class(character: char) -> &'static str {
    lookup(BIDI, u32::from(character)).map_or("L", |entry| entry.value)
}
fn general_category(character: char) -> &'static str {
    lookup(CATEGORY, u32::from(character)).map_or("Cn", |entry| entry.value)
}

fn lookup(entries: &'static [Property], code_point: u32) -> Option<&'static Property> {
    entries
        .iter()
        .find(|entry| entry.start <= code_point && code_point <= entry.end)
}

fn lookup_mapping_entry(entries: &'static [Entry], code_point: u32) -> Option<&'static Entry> {
    entries
        .iter()
        .find(|entry| entry.start <= code_point && code_point <= entry.end)
}

// Mapping and property tables share the same range shape but intentionally
// retain distinct lookups so generated passive data never exposes a public type.
fn lookup_mapping(code_point: u32) -> Option<&'static Entry> {
    lookup_mapping_entry(MAPPING, code_point)
}
