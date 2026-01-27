use core::fmt;

/// Length of a string-encoded [ULID]
pub const ULID_LEN: usize = 26;

pub(crate) const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub(crate) const NO_VALUE: u8 = 255;
pub(crate) const LOOKUP: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
    255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 255, 18, 19, 255, 20, 21, 255, 22, 23, 24,
    25, 26, 255, 27, 28, 29, 30, 31, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17,
    255, 18, 19, 255, 20, 21, 255, 22, 23, 24, 25, 26, 255, 27, 28, 29, 30, 31, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

/// An error that can occur when encoding a base32 string
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum EncodeError {
    /// The length of the provided buffer is not large enough
    BufferTooSmall,
}

impl std::error::Error for EncodeError {}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match *self {
            EncodeError::BufferTooSmall => "buffer too small",
        };
        write!(f, "{}", text)
    }
}

/// Encode an u128 value to a given buffer.
pub fn encode_to_array(mut value: u128, buffer: &mut [u8; ULID_LEN]) {
    // NOTE: This function can't be made const because mut refs aren't allowed for some reason

    for i in 0..ULID_LEN {
        buffer[ULID_LEN - 1 - i] = ALPHABET[(value & 0x1f) as usize];
        value >>= 5;
    }
}

pub fn encode(value: u128) -> String {
    let mut buffer: [u8; ULID_LEN] = [0; ULID_LEN];

    encode_to_array(value, &mut buffer);

    String::from_utf8(buffer.to_vec()).expect("unexpected failure in base32 encode for ulid")
}

/// An error that can occur when decoding a base32 string
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DecodeError {
    /// The length of the string does not match the expected length
    InvalidLength,
    /// A non-base32 character was found
    InvalidChar,
}

impl std::error::Error for DecodeError {}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match *self {
            DecodeError::InvalidLength => "invalid length",
            DecodeError::InvalidChar => "invalid character",
        };
        write!(f, "{}", text)
    }
}

pub const fn decode(encoded: &str) -> Result<u128, DecodeError> {
    if encoded.len() != ULID_LEN {
        return Err(DecodeError::InvalidLength);
    }

    let mut value: u128 = 0;

    let bytes = encoded.as_bytes();

    // Manual for loop because Range::iter() isn't const
    let mut i = 0;
    while i < ULID_LEN {
        let val = LOOKUP[bytes[i] as usize];
        if val != NO_VALUE {
            value = (value << 5) | val as u128;
        } else {
            return Err(DecodeError::InvalidChar);
        }
        i += 1;
    }

    Ok(value)
}
