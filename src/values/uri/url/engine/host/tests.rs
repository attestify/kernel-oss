//! Tests for private host parsing and serialization.
use super::{Host, Validation, ends_in_number, parse_ipv6, parse_opaque, parse_special, serialize};

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
/// Verifies all accepted WHATWG IPv4 part counts serialize canonically.
fn ipv4_part_forms_success() {
    for (input, expected) in [
        ("127", "0.0.0.127"),
        ("127.1", "127.0.0.1"),
        ("127.0.1", "127.0.0.1"),
        ("127.0.0.1", "127.0.0.1"),
        ("0xffffffff", "255.255.255.255"),
        ("0177.0.0.1", "127.0.0.1"),
        ("0x7f.0.0.1", "127.0.0.1"),
    ] {
        assert_eq!(
            parse_special(input).map(|outcome| serialize(&outcome.value)),
            Ok(expected.to_owned()),
            "{input}"
        );
    }
}

#[test]
/// Verifies IPv4 numeric dispatch distinguishes valid prefixes from arbitrary text.
fn ipv4_number_dispatch_success() {
    assert!(ends_in_number("0x"));
    assert!(!ends_in_number("0xzz"));
    assert!(!ends_in_number("0foo"));
    assert!(ends_in_number("09"));
    assert!(ends_in_number("999999999999999999999999999999999999"));
    assert_eq!(
        parse_special("0x").map(|outcome| serialize(&outcome.value)),
        Ok("0.0.0.0".to_owned())
    );
    for input in ["0xzz", "0foo"] {
        assert_eq!(
            parse_special(input).map(|outcome| serialize(&outcome.value)),
            Ok(input.to_owned()),
            "{input}"
        );
    }
    for input in ["09", "4294967296", "1.2.3.4.5", "256.0.0.1"] {
        assert!(parse_special(input).is_err(), "{input}");
    }
}

#[test]
/// Verifies percent and non-ASCII special-host validation conditions are preserved.
fn special_host_validation_conditions_success() {
    let percent_encoded = parse_special("exa%6dple.com");
    assert!(percent_encoded.is_ok_and(|outcome| {
        outcome.value == Host::Domain("example.com".to_owned())
            && outcome
                .validations
                .contains(&Validation::DomainPercentEncoded)
    }));
    let non_ascii_ipv4 = parse_special("１２７");
    assert!(non_ascii_ipv4.is_ok_and(|outcome| {
        outcome.value == Host::Ipv4(127)
            && outcome.validations.contains(&Validation::Ipv4NonAsciiInput)
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
/// Verifies opaque non-URL units remain nonfatal and use C0 percent encoding.
fn opaque_host_nonfatal_validation_success() {
    assert_eq!(
        parse_opaque("\u{000e}").map(|outcome| (serialize(&outcome.value), outcome.validations)),
        Ok(("%0E".to_owned(), vec![Validation::InvalidUrlUnit]))
    );
    assert_eq!(
        parse_opaque("%ZZ").map(|outcome| (serialize(&outcome.value), outcome.validations)),
        Ok(("%ZZ".to_owned(), vec![Validation::InvalidUrlUnit]))
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
    assert_eq!(
        serialize(&Host::Ipv6([1, 0, 2, 3, 4, 5, 6, 7])),
        "[1:0:2:3:4:5:6:7]"
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
    assert_eq!(
        parse_ipv6("1:2:3:4:5:6:192.0.2.1").map(|host| serialize(&host)),
        Ok("[1:2:3:4:5:6:c000:201]".to_owned())
    );
    assert_eq!(
        parse_ipv6("::").map(|host| serialize(&host)),
        Ok("[::]".to_owned())
    );
}

#[test]
/// Verifies malformed and zone-identified IPv6 values are rejected.
fn ipv6_invalid_error() {
    assert!(parse_ipv6("1:2:3:4:5:6:7").is_err());
    assert!(parse_ipv6("fe80::1%25en0").is_err());
    assert!(parse_ipv6("1::2::3").is_err());
    assert!(parse_ipv6("::ffff:0x7f.1").is_err());
    assert!(parse_ipv6("::ffff:127.1").is_err());
    assert!(parse_ipv6("::ffff:192.0.2.999").is_err());
    assert!(parse_ipv6("192.0.2.1::").is_err());
    assert!(parse_ipv6("1:::2").is_err());
    assert!(parse_ipv6("1:2:3:4:5:6:7:8:9").is_err());
}
