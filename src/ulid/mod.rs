// ulid/mod.rs

//! This module contains the mechanism to a ULID (Universally Unique Lexicographically Sortable Identifier) as the basis for all entity identities.
//!
//! A ULID is a 128-bit lexicographical identifier that allows for them to be sortable by nature, and highly random such that there are no identity collisions.
//!
//! View the [ULID Specification](https://github.com/ulid/spec).
//!
//! # Important Information
//!
//!  - Much of this module is copied from [ulid-rs](https://github.com/dylanhart/ulid-rs) source code, and modified to fit our needs
//! -  This source code was copied on Feb. 13, 2025, and takein from [commit 482338a](https://github.com/dylanhart/ulid-rs/tree/482338a638335ee8632cde85a79fc722fda5113e)
//!  - The licesnse this point in time was an [MIT License](https://github.com/dylanhart/ulid-rs/blob/482338a638335ee8632cde85a79fc722fda5113e/LICENSE)
//!
//! # Design Decision (Feb. 13, 2025)
//!
//! - The ULID has been chosen as the standard data format for all persistable identities because time order and time-sortability is key to answer the fundamental questions of, "What was my state or behavior on/between [some dates]."
//!  - An abstraction for the [IdentitySource] has been provided.
//!     - This replaces the [rand](https://github.com/rust-random/rand) crate to ensure there is direct dependency coupling within the kernel.

// TODO - LEFT OFF - Get all the code ported and the test working...look at their wasm test as well.

pub mod base32;

#[cfg(test)] mod base32_tests;
#[cfg(test)] mod ulid_tests;

use std::fmt;
use std::str::FromStr;
use crate::ulid::base32::{DecodeError, ULID_LEN};

/// Create a right-aligned bitmask of $len bits
#[macro_export] macro_rules! bitmask {
    ($len:expr) => {
        ((1 << $len) - 1)
    };
}

/// A [ULID] is a unique 128-bit lexicographically sortable identifier
///
/// Canonically, it is represented as a 26 character Crockford's Base32 encoded string.
///
/// - Of the 128-bits, the first 48 are a unix timestamp in milliseconds. 
/// - The  remaining 80 are random. 
/// - The first 48 provide for lexicographic sorting and the remaining 80 ensure that the identifier is unique.
///
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ULID(pub u128);

impl ULID {

    /// The number of bits in a [ULID]'s time portion
    pub const TIME_BITS: u8 = 48;
    /// The number of bits in a [ULID]'s random portion
    pub const RAND_BITS: u8 = 80;

    /// Create a [ULID] from separated parts.
    ///
    /// NOTE: Any overflow bits in the given args are discarded
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    ///
    /// let ulid = ULID::from_parts(1234567890123456789, 12345678901234567890123456789012345678);
    ///
    /// let ulid2 = ULID::from_parts(1234567890123456789, 12345678901234567890123456789012345678);
    ///
    /// assert_eq!(ulid, ulid2);
    /// ```
    pub const fn from_parts(timestamp_ms: u64, random: u128) -> ULID {
        let time_part = (timestamp_ms & bitmask!(Self::TIME_BITS)) as u128;
        let rand_part = random & bitmask!(Self::RAND_BITS);
        ULID((time_part << Self::RAND_BITS) | rand_part)
    }

    /// Creates a [ULID] from a Crockford Base32 encoded string
    ///
    /// An DecodeError will be returned when the given string is not formatted
    /// properly.
    ///
    /// # Example
    /// ```rust
    ///  use attestify_kernel::ulid::ULID;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let result = ULID::from_string(text);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(&result.unwrap().to_string(), text);
    /// ```
    pub const fn from_string(encoded: &str) -> Result<ULID, DecodeError> {
        match base32::decode(encoded) {
            Ok(int_val) => Ok(ULID(int_val)),
            Err(err) => Err(err),
        }
    }

    /// The 'nil [ULID]'.
    ///
    /// The nil [ULID] is special form of [ULID] that is specified to have all 128 bits set to zero.
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    ///
    /// let ulid = ULID::nil();
    ///
    /// assert_eq!(
    ///     ulid.to_string(),
    ///     "00000000000000000000000000"
    /// );
    /// ```
    pub const fn nil() -> ULID {
        ULID(0)
    }

    /// Gets the random section of this ulid
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let ulid = ULID::from_string(text).unwrap();
    /// let ulid_next = ulid.increment().unwrap();
    ///
    /// assert_eq!(ulid.random() + 1, ulid_next.random());
    /// ```
    pub const fn random(&self) -> u128 {
        self.0 & bitmask!(Self::RAND_BITS)
    }

    /// Creates a Crockford Base32 encoded string that represents this [ULID]
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    /// use attestify_kernel::ulid::base32:: ULID_LEN;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let ulid = ULID::from_string(text).unwrap();
    ///
    /// let mut buf = [0; ULID_LEN];
    /// let new_text = ulid.array_to_str(&mut buf);
    ///
    /// assert_eq!(new_text, text);
    /// ```
    pub fn array_to_str<'buf>(&self, buf: &'buf mut [u8; ULID_LEN]) -> &'buf mut str {
        base32::encode_to_array(self.0, buf);
        unsafe { core::str::from_utf8_unchecked_mut(buf) }
    }

    /// Creates a Crockford Base32 encoded string that represents this [ULID]
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let ulid = ULID::from_string(text).unwrap();
    ///
    /// assert_eq!(&ulid.to_string(), text);
    /// ```
    #[allow(clippy::inherent_to_string_shadow_display)]  // Significantly faster than Display::to_string
    pub fn to_string(&self) -> String {
        base32::encode(self.0)
    }

    /// Test if the [ULID] is nil
    ///
    /// # Example
    /// ```rust
    /// use attestify_kernel::ulid::ULID;
    ///
    /// # let ulid = ULID(1);
    /// # #[cfg(feature = "std")]
    /// let ulid = ULID::new();
    /// assert!(!ulid.is_nil());
    ///
    /// let nil = ULID::nil();
    /// assert!(nil.is_nil());
    /// ```
    pub const fn is_nil(&self) -> bool {
        self.0 == 0u128
    }

    /// Increment the random number, make sure that the ts millis stays the same
    pub const fn increment(&self) -> Option<ULID> {
        const MAX_RANDOM: u128 = bitmask!(ULID::RAND_BITS);

        if (self.0 & MAX_RANDOM) == MAX_RANDOM {
            None
        } else {
            Some(ULID(self.0 + 1))
        }
    }

    /// Creates a [ULID] using the provided bytes array.
    ///
    /// # Example
    /// ```
    /// use attestify_kernel::ulid::ULID;
    /// let bytes = [0xFF; 16];
    ///
    /// let ulid = ULID::from_bytes(bytes);
    ///
    /// assert_eq!(
    ///     ulid.to_string(),
    ///     "7ZZZZZZZZZZZZZZZZZZZZZZZZZ"
    /// );
    /// ```
    pub const fn from_bytes(bytes: [u8; 16]) -> ULID {
        Self(u128::from_be_bytes(bytes))
    }

    /// Returns the bytes of the [ULID] in big-endian order.
    ///
    /// # Example
    /// ```
    /// use attestify_kernel::ulid::ULID;
    ///
    /// let text = "7ZZZZZZZZZZZZZZZZZZZZZZZZZ";
    /// let ulid = ULID::from_string(text).unwrap();
    ///
    /// assert_eq!(ulid.to_bytes(), [0xFF; 16]);
    /// ```
    pub const fn to_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

}

impl Default for ULID {
    fn default() -> Self {
        ULID::nil()
    }
}

impl From<ULID> for String {
    fn from(ulid: ULID) -> String {
        ulid.to_string()
    }
}

impl From<(u64, u64)> for ULID {
    fn from((msb, lsb): (u64, u64)) -> Self {
        ULID(u128::from(msb) << 64 | u128::from(lsb))
    }
}

impl From<ULID> for (u64, u64) {
    fn from(ulid: ULID) -> (u64, u64) {
        ((ulid.0 >> 64) as u64, (ulid.0 & bitmask!(64)) as u64)
    }
}

impl From<u128> for ULID {
    fn from(value: u128) -> ULID {
        ULID(value)
    }
}

impl From<ULID> for u128 {
    fn from(ulid: ULID) -> u128 {
        ulid.0
    }
}

impl From<[u8; 16]> for ULID {
    fn from(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }
}

impl From<ULID> for [u8; 16] {
    fn from(ulid: ULID) -> Self {
        ulid.0.to_be_bytes()
    }
}

impl FromStr for ULID {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ULID::from_string(s)
    }
}

impl TryFrom<&'_ str> for ULID {
    type Error = DecodeError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        ULID::from_string(value)
    }
}

impl fmt::Display for ULID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut buffer = [0; ULID_LEN];
        write!(f, "{}", self.array_to_str(&mut buffer))
    }
}







