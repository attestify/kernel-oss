//! Tests for private host parsing and serialization.
use super::{Host, Validation, parse_ipv6, parse_opaque, parse_special, serialize};

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
/// Verifies octal, shortened, and trailing-dot IPv4 conditions are preserved.
fn ipv4_validation_conditions_success() {
    let result = parse_special("0177.1.");
    assert_eq!(
        result.as_ref().map(|outcome| serialize(&outcome.value)),
        Ok("127.0.0.1".to_owned())
    );
    assert!(result.is_ok_and(|outcome| {
        outcome.validations.contains(&Validation::Ipv4NonDecimal)
            && outcome.validations.contains(&Validation::Ipv4Shortened)
            && outcome.validations.contains(&Validation::Ipv4TrailingDot)
    }));
}

#[test]
/// Verifies special-host IDNA strict diagnostics survive relaxed compatibility.
fn strict_idna_validation_is_preserved_success() {
    let result = parse_special("under_score.example");
    assert_eq!(
        result.as_ref().map(|outcome| serialize(&outcome.value)),
        Ok("under_score.example".to_owned())
    );
    assert!(result.is_ok_and(|outcome| outcome.validations.contains(&Validation::StrictIdna)));
}

#[test]
/// Verifies invalid numeric hosts and large inputs return without panicking.
fn numeric_host_and_long_input_error() {
    assert!(parse_special("256.0.0.1").is_err());
    assert!(parse_special("1.2.3.4.5").is_err());
    assert!(parse_special(&"a".repeat(16_384)).is_ok());
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

#[test]
/// Verifies compressed and embedded-IPv4 IPv6 forms use canonical lowercase output.
fn ipv6_parsing_success() {
    assert_eq!(
        parse_ipv6("2001:0db8::1").map(|host| serialize(&host)),
        Ok("[2001:db8::1]".to_owned())
    );
    assert_eq!(
        parse_ipv6("::ffff:192.0.2.128").map(|host| serialize(&host)),
        Ok("[::ffff:c000:280]".to_owned())
    );
}

#[test]
/// Verifies malformed and zone-identified IPv6 values are rejected.
fn ipv6_invalid_error() {
    assert!(parse_ipv6("1:2:3:4:5:6:7").is_err());
    assert!(parse_ipv6("fe80::1%25en0").is_err());
    assert!(parse_ipv6("1::2::3").is_err());
}
