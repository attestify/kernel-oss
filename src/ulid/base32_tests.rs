use crate::ulid::base32::ALPHABET;
use crate::ulid::base32::DecodeError;
use crate::ulid::base32::ULID_LEN;
use crate::ulid::base32::{decode, encode};

/// Generator code for `LOOKUP`
#[test]
fn test_lookup_table() {
    let mut lookup = [crate::ulid::base32::NO_VALUE; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lookup[c as usize] = i as u8;
        if !(c as char).is_numeric() {
            //lowercase
            lookup[(c + 32) as usize] = i as u8;
        }
    }
    assert_eq!(crate::ulid::base32::LOOKUP, lookup);
}

#[test]
fn test_valid() {
    let val = 0x41414141414141414141414141414141;
    assert_eq!(decode("21850M2GA1850M2GA1850M2GA1").unwrap(), val);
    assert_eq!(encode(val), "21850M2GA1850M2GA1850M2GA1");

    let val = 0x4d4e385051444a59454234335a413756;
    let enc = "2D9RW50MA499CMAGHM6DD42DTP";
    let lower = enc.to_lowercase();
    assert_eq!(encode(val), enc);
    assert_eq!(decode(enc).unwrap(), val);
    assert_eq!(decode(&lower).unwrap(), val);
}

#[test]
fn test_length() {
    assert_eq!(encode(0xffffffffffffffffffffffffffffffff).len(), ULID_LEN);
    assert_eq!(encode(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f).len(), ULID_LEN);
    assert_eq!(encode(0x00000000000000000000000000000000).len(), ULID_LEN);

    assert_eq!(decode(""), Err(DecodeError::InvalidLength));
    assert_eq!(
        decode("2D9RW50MA499CMAGHM6DD42DT"),
        Err(DecodeError::InvalidLength)
    );
    assert_eq!(
        decode("2D9RW50MA499CMAGHM6DD42DTPP"),
        Err(DecodeError::InvalidLength)
    );
}

#[test]
fn test_chars() {
    for ref c in encode(0xffffffffffffffffffffffffffffffff).bytes() {
        assert!(ALPHABET.contains(c));
    }
    for ref c in encode(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f).bytes() {
        assert!(ALPHABET.contains(c));
    }
    for ref c in encode(0x00000000000000000000000000000000).bytes() {
        assert!(ALPHABET.contains(c));
    }

    assert_eq!(
        decode("2D9RW50[A499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
    assert_eq!(
        decode("2D9RW50LA499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
    assert_eq!(
        decode("2D9RW50IA499CMAGHM6DD42DTP"),
        Err(DecodeError::InvalidChar)
    );
}
