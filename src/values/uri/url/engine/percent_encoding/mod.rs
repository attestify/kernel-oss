//! Private WHATWG percent-encoding and byte-decoding primitives.
#![cfg_attr(
    not(test),
    expect(dead_code, reason = "P3 private primitives are wired by P4 parsing.")
)]

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
pub(in crate::values::uri::url) enum EncodeSet {
    C0Control,
    Fragment,
    Query,
    SpecialQuery,
    Path,
    UserInfo,
    Component,
    FormUrlEncoded,
}

/// Encodes UTF-8 bytes using the selected WHATWG percent-encode set.
pub(in crate::values::uri::url) fn encode(input: &str, set: EncodeSet) -> String {
    let mut output = String::new();
    for byte in input.bytes() {
        if must_encode(byte, set) {
            output.push('%');
            output.push(hex(byte >> 4));
            output.push(hex(byte & 0x0f));
        } else {
            output.push(char::from(byte));
        }
    }
    output
}

/// Decodes valid percent triplets exactly once, retaining malformed bytes.
pub(super) fn decode_bytes(input: &str) -> Vec<u8> {
    let bytes = input.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        if byte == b'%'
            && let (Some(&high), Some(&low)) = (bytes.get(index + 1), bytes.get(index + 2))
            && let (Some(high), Some(low)) = (from_hex(high), from_hex(low))
        {
            output.push((high << 4) | low);
            index += 3;
            continue;
        }
        output.push(byte);
        index += 1;
    }
    output
}

/// Performs UTF-8 decoding with replacement and no BOM special handling.
pub(super) fn decode_utf8_replacement(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Performs fail-fast UTF-8 decoding without exposing encoding details.
pub(super) fn decode_utf8(bytes: &[u8]) -> Result<&str, ()> {
    core::str::from_utf8(bytes).map_err(|_| ())
}

fn must_encode(byte: u8, set: EncodeSet) -> bool {
    let c0 = byte <= 0x1f || byte > 0x7e;
    if c0 {
        return true;
    }
    match set {
        EncodeSet::C0Control => false,
        EncodeSet::Fragment => matches!(byte, b' ' | b'"' | b'<' | b'>' | 0x60),
        EncodeSet::Query => matches!(byte, b' ' | b'"' | b'#' | b'<' | b'>'),
        EncodeSet::SpecialQuery => matches!(byte, b' ' | b'"' | b'#' | b'<' | b'>' | b'\''),
        EncodeSet::Path => matches!(
            byte,
            b' ' | b'"' | b'#' | b'<' | b'>' | b'?' | 0x60 | b'{' | b'}' | b'^'
        ),
        EncodeSet::UserInfo => matches!(
            byte,
            b' ' | b'"'
                | b'#'
                | b'<'
                | b'>'
                | b'?'
                | 0x60
                | b'{'
                | b'}'
                | b'/'
                | b':'
                | b';'
                | b'='
                | b'@'
                | b'['
                | b'\\'
                | b']'
                | b'^'
                | b'|'
        ),
        EncodeSet::Component => !matches!(
            byte,
            b'A'..=b'Z'
                | b'a'..=b'z'
                | b'0'..=b'9'
                | b'!'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'-'
                | b'.'
                | b'_'
                | b'~'
        ),
        EncodeSet::FormUrlEncoded => {
            !matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_')
        }
    }
}

fn hex(value: u8) -> char {
    match value {
        0..=9 => char::from(b'0' + value),
        _ => char::from(b'A' + value - 10),
    }
}

fn from_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
