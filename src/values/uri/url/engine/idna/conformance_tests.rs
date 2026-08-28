//! Exhaustive private Unicode and WPT IDNA conformance tests.

use super::{
    domain_to_ascii, fixture_vectors, raw_to_unicode, to_ascii, wpt_idna_v2_vectors, wpt_vectors,
};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies all applicable no-error Unicode 17 UTS #46 vectors.
#[test]
fn unicode_17_uts46_success_vectors_success() {
    for (input, expected) in fixture_vectors::UTS46_SUCCESS_CASES {
        assert_eq!(
            to_ascii(input, false),
            Ok((*expected).to_owned()),
            "{input:?}"
        );
    }
    assert_eq!(
        fixture_vectors::UTS46_SUCCESS_CASE_COUNT,
        fixture_vectors::UTS46_SUCCESS_CASES.len()
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Records that only ill-formed UTF-16 source vectors are inapplicable to Rust strings.
#[test]
fn unicode_17_ill_formed_utf16_vectors_are_recorded_success() {
    assert_eq!(fixture_vectors::UTS46_ILL_FORMED_SKIPPED_COUNT, 2);
    assert_eq!(
        fixture_vectors::UTS46_SUCCESS_CASE_COUNT
            + fixture_vectors::UTS46_ERROR_CASE_COUNT
            + fixture_vectors::UTS46_TO_UNICODE_ONLY_COUNT
            + fixture_vectors::UTS46_ILL_FORMED_SKIPPED_COUNT,
        6_391
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies every Unicode 17 vector that requests a non-optional failure fails.
#[test]
fn unicode_17_uts46_error_vectors_error() {
    for input in fixture_vectors::UTS46_ERROR_CASES {
        assert!(to_ascii(input, false).is_err(), "{input:?}");
    }
    assert_eq!(
        fixture_vectors::UTS46_ERROR_CASE_COUNT,
        fixture_vectors::UTS46_ERROR_CASES.len()
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies status-bearing UTS #46 ToUnicode vectors retain conversion and error state.
#[test]
fn unicode_17_uts46_to_unicode_vectors_success() {
    for (input, expected, has_error) in fixture_vectors::UTS46_TO_UNICODE_CASES {
        let outcome = raw_to_unicode(input);
        assert_eq!(outcome.value, *expected, "{input:?}");
        assert_eq!(outcome.error.is_some(), *has_error, "{input:?}");
    }
    assert_eq!(
        fixture_vectors::UTS46_TO_UNICODE_ONLY_COUNT,
        fixture_vectors::UTS46_TO_UNICODE_CASES.len()
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies all Rust-representable WPT WHATWG ToASCII vectors.
#[test]
fn wpt_toascii_vectors_success() {
    for (input, expected, url_standard_only) in wpt_vectors::WPT_TOASCII_CASES {
        let actual = if *url_standard_only || input.is_ascii() {
            domain_to_ascii(input, false)
        } else {
            to_ascii(input, false)
        };
        match expected {
            Some(expected) => assert_eq!(actual, Ok((*expected).to_owned()), "{input:?}"),
            None => assert!(actual.is_err(), "{input:?}"),
        }
    }
    assert_eq!(
        wpt_vectors::WPT_TOASCII_CASE_COUNT,
        wpt_vectors::WPT_TOASCII_CASES.len()
    );
    assert_eq!(wpt_vectors::WPT_TOASCII_ILL_FORMED_SKIPPED_COUNT, 0);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies every Rust-representable WPT IdnaTestV2 domain-to-ASCII case.
#[test]
fn wpt_idna_v2_vectors_success() {
    for (input, expected) in wpt_idna_v2_vectors::WPT_IDNA_V2_CASES {
        match expected {
            Some(expected) if input.is_empty() && expected.is_empty() => {
                assert_eq!(to_ascii(input, false), Ok(String::new()));
            }
            Some(expected) => assert_eq!(
                domain_to_ascii(input, false),
                Ok((*expected).to_owned()),
                "{input:?}"
            ),
            None => assert!(domain_to_ascii(input, false).is_err(), "{input:?}"),
        }
    }
    assert_eq!(
        wpt_idna_v2_vectors::WPT_IDNA_V2_CASE_COUNT,
        wpt_idna_v2_vectors::WPT_IDNA_V2_CASES.len()
    );
    assert_eq!(wpt_idna_v2_vectors::WPT_IDNA_V2_ILL_FORMED_SKIPPED_COUNT, 2);
}
