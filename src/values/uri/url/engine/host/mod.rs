//! Private WHATWG host parsing, validation outcomes, and IP serialization.
#![cfg_attr(
    not(test),
    expect(dead_code, reason = "P3 private primitives are wired by P4 parsing.")
)]

#[cfg(test)]
mod tests;

use super::{idna, percent_encoding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Host {
    Domain(String),
    Ipv4(u32),
    Ipv6([u16; 8]),
    Opaque(String),
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Validation {
    StrictIdna,
    Ipv4NonDecimal,
    Ipv4Shortened,
    Ipv4TrailingDot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Outcome {
    pub(super) value: Host,
    pub(super) validations: Vec<Validation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Invalid,
    Overflow,
}

/// Parses a special-scheme host through percent decoding, IDNA, and IPv4 dispatch.
pub(super) fn parse_special(input: &str) -> Result<Outcome, Error> {
    let bytes = percent_encoding::decode_bytes(input);
    let decoded = percent_encoding::decode_utf8(&bytes).map_err(|_| Error::Invalid)?;
    let strict_failed = idna::to_ascii(decoded, true).is_err();
    let domain = idna::domain_to_ascii(decoded, false).map_err(|_| Error::Invalid)?;
    let mut validations = Vec::new();
    if strict_failed {
        validations.push(Validation::StrictIdna);
    }
    if ends_in_number(&domain) {
        let (value, mut ipv4_validations) = parse_ipv4(&domain)?;
        validations.append(&mut ipv4_validations);
        return Ok(Outcome {
            value: Host::Ipv4(value),
            validations,
        });
    }
    Ok(Outcome {
        value: Host::Domain(domain),
        validations,
    })
}

/// Parses an opaque host without IDNA or numeric-host interpretation.
pub(super) fn parse_opaque(input: &str) -> Result<Outcome, Error> {
    if input.chars().any(forbidden_opaque) {
        return Err(Error::Invalid);
    }
    Ok(Outcome {
        value: Host::Opaque(super::percent_encoding::encode(
            input,
            super::percent_encoding::EncodeSet::C0Control,
        )),
        validations: Vec::new(),
    })
}

/// Parses one bracket-content IPv6 address into exactly eight numeric pieces.
pub(super) fn parse_ipv6(input: &str) -> Result<Host, Error> {
    if input.is_empty() || input.contains('%') || input.matches("::").count() > 1 {
        return Err(Error::Invalid);
    }
    let (left, right, compressed) = if let Some((left, right)) = input.split_once("::") {
        (left, right, true)
    } else {
        (input, "", false)
    };
    let left = parse_ipv6_side(left)?;
    let right = parse_ipv6_side(right)?;
    let count = left.len().checked_add(right.len()).ok_or(Error::Overflow)?;
    if count > 8 || (!compressed && count != 8) || (compressed && count == 8) {
        return Err(Error::Invalid);
    }
    let mut pieces = [0u16; 8];
    for (index, piece) in left.iter().enumerate() {
        pieces[index] = *piece;
    }
    let right_start = 8usize.checked_sub(right.len()).ok_or(Error::Overflow)?;
    for (offset, piece) in right.iter().enumerate() {
        pieces[right_start.checked_add(offset).ok_or(Error::Overflow)?] = *piece;
    }
    Ok(Host::Ipv6(pieces))
}

pub(super) fn serialize(host: &Host) -> String {
    match host {
        Host::Domain(value) | Host::Opaque(value) => value.clone(),
        Host::Ipv4(value) => format!(
            "{}.{}.{}.{}",
            value >> 24,
            (value >> 16) & 255,
            (value >> 8) & 255,
            value & 255
        ),
        Host::Ipv6(pieces) => format!("[{}]", serialize_ipv6(pieces)),
        Host::Empty => String::new(),
    }
}

fn parse_ipv6_side(side: &str) -> Result<Vec<u16>, Error> {
    if side.is_empty() {
        return Ok(Vec::new());
    }
    let segments: Vec<&str> = side.split(':').collect();
    let mut pieces = Vec::new();
    for (index, segment) in segments.iter().enumerate() {
        if segment.is_empty() {
            return Err(Error::Invalid);
        }
        if segment.contains('.') {
            if index + 1 != segments.len() {
                return Err(Error::Invalid);
            }
            let (address, _) = parse_ipv4(segment)?;
            pieces.push(u16::try_from(address >> 16).map_err(|_| Error::Overflow)?);
            pieces.push(u16::try_from(address & 0xffff).map_err(|_| Error::Overflow)?);
        } else if segment.len() > 4 {
            return Err(Error::Invalid);
        } else {
            pieces.push(u16::from_str_radix(segment, 16).map_err(|_| Error::Invalid)?);
        }
    }
    Ok(pieces)
}

fn ends_in_number(domain: &str) -> bool {
    let label = domain
        .strip_suffix('.')
        .unwrap_or(domain)
        .rsplit('.')
        .next()
        .unwrap_or_default();
    label.parse::<u32>().is_ok()
        || label.starts_with("0x")
        || label.starts_with("0X")
        || label.starts_with('0')
}

fn parse_ipv4(input: &str) -> Result<(u32, Vec<Validation>), Error> {
    let mut text = input;
    let mut validations = Vec::new();
    if let Some(trimmed) = text.strip_suffix('.') {
        text = trimmed;
        validations.push(Validation::Ipv4TrailingDot);
    }
    let parts: Vec<&str> = text.split('.').collect();
    if parts.is_empty() || parts.len() > 4 || parts.iter().any(|part| part.is_empty()) {
        return Err(Error::Invalid);
    }
    if parts.len() < 4 {
        validations.push(Validation::Ipv4Shortened);
    }
    let mut values = Vec::new();
    for part in parts {
        let (radix, digits, non_decimal) =
            if let Some(value) = part.strip_prefix("0x").or_else(|| part.strip_prefix("0X")) {
                (16, value, true)
            } else if part.len() > 1 && part.starts_with('0') {
                (8, &part[1..], true)
            } else {
                (10, part, false)
            };
        if digits.is_empty() {
            return Err(Error::Invalid);
        }
        if non_decimal {
            validations.push(Validation::Ipv4NonDecimal);
        }
        values.push(u32::from_str_radix(digits, radix).map_err(|_| Error::Invalid)?);
    }
    if values[..values.len() - 1].iter().any(|value| *value > 255) {
        return Err(Error::Invalid);
    }
    let last_limit = 1u32
        .checked_shl(8 * (5 - u32::try_from(values.len()).map_err(|_| Error::Overflow)?))
        .ok_or(Error::Overflow)?;
    if *values.last().ok_or(Error::Invalid)? >= last_limit {
        return Err(Error::Invalid);
    }
    let mut value = *values.last().ok_or(Error::Invalid)?;
    for (index, part) in values[..values.len() - 1].iter().enumerate() {
        let shift = 8 * (3 - u32::try_from(index).map_err(|_| Error::Overflow)?);
        value = value
            .checked_add(part.checked_shl(shift).ok_or(Error::Overflow)?)
            .ok_or(Error::Overflow)?;
    }
    Ok((value, validations))
}

fn forbidden_opaque(character: char) -> bool {
    u32::from(character) <= 0x20
        || matches!(
            character,
            '#' | '/' | ':' | '<' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '|'
        )
}

fn serialize_ipv6(pieces: &[u16; 8]) -> String {
    let mut best = None;
    let mut index = 0;
    while index < 8 {
        if pieces[index] != 0 {
            index += 1;
            continue;
        }
        let start = index;
        while index < 8 && pieces[index] == 0 {
            index += 1;
        }
        if index - start >= 2 && best.is_none_or(|(_, length)| index - start > length) {
            best = Some((start, index - start));
        }
    }
    let mut output = String::new();
    let mut index = 0;
    while index < 8 {
        if let Some((start, length)) = best
            && index == start
        {
            output.push_str("::");
            index += length;
            continue;
        }
        if !output.is_empty() && !output.ends_with(':') {
            output.push(':');
        }
        output.push_str(&format!("{:x}", pieces[index]));
        index += 1;
    }
    output
}
