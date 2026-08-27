//! Tests for private host parsing and serialization.
use super::{Host, Validation, parse_opaque, parse_special, serialize};

#[test]
/// Verifies shortened hexadecimal IPv4 uses canonical dotted decimal.
fn ipv4_non_decimal_success() {
    let result = parse_special("0x7f.1");
    assert_eq!(
        result.as_ref().map(|outcome| serialize(&outcome.value)),
        Ok("127.0.0.1".to_owned())
    );
    assert!(result.is_ok_and(|outcome| outcome.validations.contains(&Validation::Ipv4NonDecimal)));
}
#[test]
/// Verifies opaque hosts do not apply IDNA or IPv4 parsing.
fn opaque_host_success() {
    assert_eq!(
        parse_opaque("0x7f.1").map(|outcome| outcome.value),
        Ok(Host::Opaque("0x7f.1".to_owned()))
    );
}
#[test]
/// Verifies forbidden opaque host characters fail.
fn opaque_host_forbidden_error() {
    assert!(parse_opaque("a/b").is_err());
}
#[test]
/// Verifies IPv6 serialization chooses the first longest zero run.
fn ipv6_serialization_success() {
    assert_eq!(
        serialize(&Host::Ipv6([1, 0, 0, 2, 0, 0, 3, 4])),
        "[1::2:0:0:3:4]"
    );
    assert_eq!(serialize(&Host::Empty), "");
}
