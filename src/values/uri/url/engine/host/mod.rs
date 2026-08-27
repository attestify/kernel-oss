//! Private WHATWG host parsing, validation outcomes, and IP serialization.

#[cfg(test)]
mod tests;

use super::{idna, percent_encoding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::values::uri::url) enum Host {
    Domain(String),
    Ipv4(u32),
    Ipv6([u16; 8]),
    Opaque(String),
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Validation {
    StrictIdna,
    DomainPercentEncoded,
    Ipv4NonAsciiInput,
    Ipv4NonDecimal,
    Ipv4Shortened,
    Ipv4TrailingDot,
    Ipv6PieceLeadingZero,
    InvalidUrlUnit,
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
    let decoded_has_non_ascii = !decoded.is_ascii();
    let strict_failed = idna::to_ascii(decoded, true).is_err();
    let domain = idna::domain_to_ascii(decoded, false).map_err(|_| Error::Invalid)?;
    let mut validations = Vec::new();
    if strict_failed {
        validations.push(Validation::StrictIdna);
    }
    if has_valid_percent_triplet(input) {
        validations.push(Validation::DomainPercentEncoded);
    }
    if ends_in_number(&domain) {
        if decoded_has_non_ascii {
            validations.push(Validation::Ipv4NonAsciiInput);
        }
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
    let mut validations = Vec::new();
    if input.chars().any(non_url_code_point) || has_malformed_percent_triplet(input) {
        validations.push(Validation::InvalidUrlUnit);
    }
    Ok(Outcome {
        value: Host::Opaque(super::percent_encoding::encode(
            input,
            super::percent_encoding::EncodeSet::C0Control,
        )),
        validations,
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
    let left = parse_ipv6_side(left, !compressed && right.is_empty())?;
    let right = parse_ipv6_side(right, true)?;
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

/// Identifies the nonfatal URL validation condition for a hexadecimal IPv6 piece
/// with an unnecessary leading zero. Embedded IPv4 is governed separately.
pub(super) fn ipv6_piece_has_leading_zero(input: &str) -> bool {
    input.split(':').any(|piece| {
        !piece.contains('.') && piece.len() > 1 && piece.as_bytes().first() == Some(&b'0')
    })
}

pub(in crate::values::uri::url) fn serialize(host: &Host) -> String {
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

fn parse_ipv6_side(side: &str, embedded_ipv4_allowed: bool) -> Result<Vec<u16>, Error> {
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
            if !embedded_ipv4_allowed || index + 1 != segments.len() {
                return Err(Error::Invalid);
            }
            let [first, second] = parse_embedded_ipv4(segment)?;
            pieces.push(first);
            pieces.push(second);
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
    !label.is_empty()
        && (label.bytes().all(|byte| byte.is_ascii_digit())
            || parse_ipv4_number(label).is_ok()
            // A syntactically numeric hexadecimal label still enters the IPv4
            // parser even if its value overflows there. That lets the parser
            // reject it as an invalid IPv4 host rather than treating it as a
            // domain. Prefixes with invalid digits (for example `0xzz`) do not
            // take this path.
            || label
                .strip_prefix("0x")
                .or_else(|| label.strip_prefix("0X"))
                .is_some_and(|digits| {
                    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
                }))
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
        let (value, non_decimal) = parse_ipv4_number(part)?;
        if non_decimal {
            validations.push(Validation::Ipv4NonDecimal);
        }
        values.push(value);
    }
    if values[..values.len() - 1].iter().any(|value| *value > 255) {
        return Err(Error::Invalid);
    }
    let part_count = values.len();
    let last_maximum = if part_count == 1 {
        u64::from(u32::MAX)
    } else {
        let part_count = u32::try_from(part_count).map_err(|_| Error::Overflow)?;
        let remaining = 5u32.checked_sub(part_count).ok_or(Error::Overflow)?;
        let shift = 8u32.checked_mul(remaining).ok_or(Error::Overflow)?;
        1u64.checked_shl(shift).ok_or(Error::Overflow)? - 1
    };
    if *values.last().ok_or(Error::Invalid)? > last_maximum {
        return Err(Error::Invalid);
    }
    let mut value = *values.last().ok_or(Error::Invalid)?;
    for (index, part) in values[..values.len() - 1].iter().enumerate() {
        let index = u32::try_from(index).map_err(|_| Error::Overflow)?;
        let remaining = 3u32.checked_sub(index).ok_or(Error::Overflow)?;
        let shift = 8u32.checked_mul(remaining).ok_or(Error::Overflow)?;
        value = value
            .checked_add(part.checked_shl(shift).ok_or(Error::Overflow)?)
            .ok_or(Error::Overflow)?;
    }
    Ok((
        u32::try_from(value).map_err(|_| Error::Overflow)?,
        validations,
    ))
}

fn forbidden_opaque(character: char) -> bool {
    matches!(
        character,
        '\0' | '\t'
            | '\n'
            | '\r'
            | ' '
            | '#'
            | '/'
            | ':'
            | '<'
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '|'
    )
}

fn non_url_code_point(character: char) -> bool {
    let value = u32::from(character);
    let noncharacter =
        (0xfdd0..=0xfdef).contains(&value) || matches!(value & 0xffff, 0xfffe | 0xffff);
    noncharacter
        || !(character.is_ascii_alphanumeric()
            || matches!(
                character,
                '!' | '$'
                    | '&'
                    | '\''
                    | '('
                    | ')'
                    | '*'
                    | '+'
                    | ','
                    | '-'
                    | '.'
                    | '/'
                    | ':'
                    | ';'
                    | '='
                    | '?'
                    | '@'
                    | '_'
                    | '~'
            )
            || value >= 0x00a0)
}

fn has_valid_percent_triplet(input: &str) -> bool {
    input.as_bytes().windows(3).any(|window| {
        window[0] == b'%' && from_hex(window[1]).is_some() && from_hex(window[2]).is_some()
    })
}

fn has_malformed_percent_triplet(input: &str) -> bool {
    let bytes = input.as_bytes();
    bytes.iter().enumerate().any(|(index, byte)| {
        *byte == b'%'
            && !matches!(
                (bytes.get(index + 1), bytes.get(index + 2)),
                (Some(high), Some(low)) if from_hex(*high).is_some() && from_hex(*low).is_some()
            )
    })
}

fn parse_ipv4_number(input: &str) -> Result<(u64, bool), Error> {
    let (radix, digits, non_decimal) = if let Some(value) = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
    {
        (16u8, value, true)
    } else if input.len() > 1 && input.starts_with('0') {
        (8u8, &input[1..], true)
    } else {
        (10u8, input, false)
    };
    if digits.is_empty() {
        return if non_decimal && radix == 16 {
            Ok((0, true))
        } else {
            Err(Error::Invalid)
        };
    }
    let mut value = 0u64;
    for byte in digits.bytes() {
        let digit = match byte {
            b'0'..=b'9' => u64::from(byte - b'0'),
            b'a'..=b'f' => u64::from(byte - b'a' + 10),
            b'A'..=b'F' => u64::from(byte - b'A' + 10),
            _ => return Err(Error::Invalid),
        };
        if digit >= u64::from(radix) {
            return Err(Error::Invalid);
        }
        value = value
            .checked_mul(u64::from(radix))
            .and_then(|current| current.checked_add(digit))
            .ok_or(Error::Overflow)?;
    }
    Ok((value, non_decimal))
}

fn parse_embedded_ipv4(input: &str) -> Result<[u16; 2], Error> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() != 4 || parts.iter().any(|part| part.is_empty()) {
        return Err(Error::Invalid);
    }
    let mut octets = [0u8; 4];
    for (index, part) in parts.iter().enumerate() {
        if part.len() > 1 && part.starts_with('0') {
            return Err(Error::Invalid);
        }
        let mut value = 0u16;
        for byte in part.bytes() {
            if !byte.is_ascii_digit() {
                return Err(Error::Invalid);
            }
            value = value
                .checked_mul(10)
                .and_then(|current| current.checked_add(u16::from(byte - b'0')))
                .ok_or(Error::Overflow)?;
            if value > 255 {
                return Err(Error::Invalid);
            }
        }
        octets[index] = u8::try_from(value).map_err(|_| Error::Overflow)?;
    }
    Ok([
        (u16::from(octets[0]) << 8) | u16::from(octets[1]),
        (u16::from(octets[2]) << 8) | u16::from(octets[3]),
    ])
}

fn from_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
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
