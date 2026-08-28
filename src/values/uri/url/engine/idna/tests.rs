//! Tests for the private dependency-free IDNA boundary.
use super::{domain_to_ascii, punycode, to_ascii, to_unicode};

#[test]
/// Verifies Unicode labels encode to canonical A-labels.
fn unicode_label_to_ascii_success() {
    assert_eq!(
        to_ascii("bücher.example", true),
        Ok("xn--bcher-kva.example".to_string())
    );
}
#[test]
/// Verifies forbidden domain code points are rejected.
fn forbidden_domain_code_point_error() {
    assert!(domain_to_ascii("a/b", true).is_err());
}

#[test]
/// Verifies mapped Unicode dot separators retain domain boundaries.
fn mapped_dot_separator_success() {
    assert_eq!(
        to_ascii("example\u{3002}COM", false),
        Ok("example.com".to_owned())
    );
}

#[test]
/// Verifies composed and decomposed labels use the shared NFC seam.
fn shared_nfc_composition_success() {
    assert_eq!(
        to_ascii("bu\u{308}cher", false),
        Ok("xn--bcher-kva".to_owned())
    );
}

#[test]
/// Verifies leading marks are rejected even when their combining class is zero.
fn leading_mark_error() {
    assert!(to_ascii("\u{093e}example", false).is_err());
}

#[test]
/// Verifies canonical A-labels decode through the private ToUnicode path.
fn a_label_to_unicode_success() {
    assert_eq!(
        to_unicode("xn--bcher-kva.example", true),
        Ok("bücher.example".to_owned())
    );
}

#[test]
/// Verifies a joiner after a shared-NFC virama is accepted.
fn context_j_virama_success() {
    assert!(to_ascii("\u{0915}\u{094d}\u{200d}\u{0937}", false).is_ok());
}

#[test]
/// Verifies a joiner without the required virama context is rejected.
fn context_j_missing_virama_error() {
    assert!(to_ascii("a\u{200d}b", false).is_err());
}

#[test]
/// Verifies right-to-left labels cannot mix European and Arabic numbers.
fn bidi_mixed_number_types_error() {
    assert!(to_ascii("\u{0627}\u{0661}1", false).is_err());
}

#[test]
/// Verifies relaxed processing retains otherwise empty mapped domain labels.
fn relaxed_empty_label_success() {
    assert_eq!(to_ascii("a..b", false), Ok("a..b".to_owned()));
}

#[test]
/// Verifies strict processing rejects an empty label.
fn strict_empty_label_error() {
    assert!(to_ascii("a..b", true).is_err());
}

#[test]
/// Verifies strict DNS label length enforcement is opt-in.
fn strict_dns_label_length_error() {
    let label = "a".repeat(64);
    assert!(to_ascii(&label, true).is_err());
    assert!(to_ascii(&label, false).is_ok());
}

#[test]
/// Verifies strict WHATWG processing applies the STD3 ASCII restriction.
fn strict_std3_underscore_error() {
    assert_eq!(
        to_ascii("under_score.example", true),
        Err(super::Error::Invalid)
    );
}

#[test]
/// Verifies relaxed ASCII compatibility keeps non-forbidden underscore labels.
fn relaxed_ascii_underscore_compatibility_success() {
    assert_eq!(
        domain_to_ascii("under_score.example", false),
        Ok("under_score.example".to_owned())
    );
}

#[test]
/// Verifies strict processing validates a decoded A-label's leading hyphen.
fn strict_decoded_a_label_leading_hyphen_error() {
    let payload = punycode::encode("-é");
    assert!(payload.is_ok());
    if let Ok(payload) = payload {
        assert_eq!(
            to_ascii(&format!("xn--{payload}"), true),
            Err(super::Error::Invalid)
        );
    }
}

#[test]
/// Verifies strict processing validates a decoded A-label's third/fourth hyphens.
fn strict_decoded_a_label_third_fourth_hyphen_error() {
    let payload = punycode::encode("ab--é");
    assert!(payload.is_ok());
    if let Ok(payload) = payload {
        assert_eq!(
            to_ascii(&format!("xn--{payload}"), true),
            Err(super::Error::Invalid)
        );
    }
}

#[test]
/// Verifies repeated processing is deterministic and idempotent.
fn to_ascii_idempotence_success() {
    let first = to_ascii("Faß.Example", false);
    assert_eq!(first, Ok("xn--fa-hia.example".to_owned()));
    if let Ok(value) = first {
        assert_eq!(to_ascii(&value, false), Ok(value));
    } else {
        panic!("private test setup must produce a valid IDNA value");
    }
}

#[test]
/// Verifies relaxed UTS #46 retains the empty converted value before wrapping.
fn relaxed_uts46_empty_input_success() {
    assert_eq!(to_ascii("", false), Ok(String::new()));
}

#[test]
/// Verifies strict UTS #46 rejects the empty DNS-length input.
fn strict_uts46_empty_input_error() {
    assert_eq!(to_ascii("", true), Err(super::Error::Invalid));
}

#[test]
/// Verifies the final relaxed WHATWG wrapper rejects an empty intermediate value.
fn relaxed_whatwg_empty_input_error() {
    assert_eq!(domain_to_ascii("", false), Err(super::Error::Invalid));
}

#[test]
/// Verifies the final strict WHATWG wrapper fails through strict UTS #46.
fn strict_whatwg_empty_input_error() {
    assert_eq!(domain_to_ascii("", true), Err(super::Error::Invalid));
}

#[test]
/// Verifies raw UTS #46 retains decoded A-label ContextJ validation.
fn raw_uts46_decoded_a_label_context_j_error() {
    assert_eq!(to_ascii("xn--ab-j1t", false), Err(super::Error::Context));
}

#[test]
/// Verifies strict WHATWG processing rejects the decoded A-label ContextJ error.
fn strict_whatwg_decoded_a_label_context_j_error() {
    assert_eq!(
        domain_to_ascii("xn--ab-j1t", true),
        Err(super::Error::Context)
    );
}

#[test]
/// Verifies relaxed WHATWG retains an entirely ASCII original domain.
fn relaxed_whatwg_ascii_a_label_success() {
    assert_eq!(
        domain_to_ascii("xn--ab-j1t", false),
        Ok("xn--ab-j1t".to_owned())
    );
}

#[test]
/// Verifies relaxed ASCII compatibility still applies final forbidden-code-point rejection.
fn relaxed_whatwg_ascii_percent_error() {
    assert_eq!(
        domain_to_ascii("xn---%-u4o.xn--gff52t", false),
        Err(super::Error::Invalid)
    );
}

#[test]
/// Verifies relaxed WHATWG does not bypass ContextJ for non-ASCII input.
fn relaxed_whatwg_non_ascii_context_j_error() {
    assert_eq!(
        domain_to_ascii("a\u{200c}b", false),
        Err(super::Error::Context)
    );
}
