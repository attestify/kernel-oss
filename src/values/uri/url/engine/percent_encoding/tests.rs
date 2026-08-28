//! Tests for private WHATWG percent processing.

use super::{EncodeSet, decode_bytes, decode_utf8_replacement, encode};

#[test]
/// Verifies adjacent encode-set boundary characters remain distinct.
fn encode_set_boundaries_success() {
    assert_eq!(encode("#'", EncodeSet::Query), "%23'");
    assert_eq!(encode("#'", EncodeSet::SpecialQuery), "%23%27");
    assert_eq!(encode("?^", EncodeSet::Path), "%3F%5E");
    assert_eq!(encode("/", EncodeSet::UserInfo), "%2F");
    assert_eq!(
        encode(&char::from(0x60).to_string(), EncodeSet::Fragment),
        "%60"
    );
    assert_eq!(encode("!\'()*-._~", EncodeSet::Component), "!\'()*-._~");
    assert_eq!(encode("$%&+,", EncodeSet::Component), "%24%25%26%2B%2C");
    assert_eq!(encode("~", EncodeSet::Component), "~");
    assert_eq!(encode("~", EncodeSet::FormUrlEncoded), "%7E");
    assert_eq!(
        encode("!\'()~", EncodeSet::FormUrlEncoded),
        "%21%27%28%29%7E"
    );
    assert_eq!(
        encode("$%&+,/:;=@[]\\|", EncodeSet::Path),
        "$%&+,/:;=@[]\\|"
    );
    assert_eq!(
        encode("$%&+,/:;=@[]\\|", EncodeSet::Component),
        "%24%25%26%2B%2C%2F%3A%3B%3D%40%5B%5D%5C%7C"
    );
    assert_eq!(encode("*-. _", EncodeSet::FormUrlEncoded), "*-.%20_");
}

#[test]
/// Verifies malformed sequences remain bytes and valid sequences decode once.
fn decode_once_success() {
    assert_eq!(
        decode_utf8_replacement(&decode_bytes("%2526%ZZ%")),
        "%26%ZZ%"
    );
}
