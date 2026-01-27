use crate::ulid::ULID;
use crate::ulid::base32::DecodeError;
use crate::ulid::base32::EncodeError;
use std::str::FromStr;

#[test]
fn test_static() {
    let s = ULID(0x41414141414141414141414141414141).to_string();
    let u = ULID::from_string(&s).unwrap();
    assert_eq!(&s, "21850M2GA1850M2GA1850M2GA1");
    assert_eq!(u.0, 0x41414141414141414141414141414141);
}

#[test]
fn test_increment() {
    let ulid = ULID::from_string("01BX5ZZKBKAZZZZZZZZZZZZZZZ").unwrap();
    let ulid = ulid.increment().unwrap();
    assert_eq!("01BX5ZZKBKB000000000000000", ulid.to_string());

    let ulid = ULID::from_string("01BX5ZZKBKZZZZZZZZZZZZZZZX").unwrap();
    let ulid = ulid.increment().unwrap();
    assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZY", ulid.to_string());
    let ulid = ulid.increment().unwrap();
    assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZZ", ulid.to_string());
    assert!(ulid.increment().is_none());
}

#[test]
fn test_increment_overflow() {
    let ulid = ULID(u128::max_value());
    assert!(ulid.increment().is_none());
}

#[test]
fn can_into_thing() {
    let ulid = ULID::from_str("01FKMG6GAG0PJANMWFN84TNXCD").unwrap();
    let s: String = ulid.into();
    let u: u128 = ulid.into();
    let uu: (u64, u64) = ulid.into();
    let bytes: [u8; 16] = ulid.into();

    assert_eq!(ULID::from_str(&s).unwrap(), ulid);
    assert_eq!(ULID::from(u), ulid);
    assert_eq!(ULID::from(uu), ulid);
    assert_eq!(ULID::from(bytes), ulid);
}

#[test]
fn default_is_nil() {
    assert_eq!(ULID::default(), ULID::nil());
}

#[test]
fn can_display_things() {
    println!("{}", ULID::nil());
    println!("{}", EncodeError::BufferTooSmall);
    println!("{}", DecodeError::InvalidLength);
    println!("{}", DecodeError::InvalidChar);
}
