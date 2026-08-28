//! Exhaustive private percent-encoding conformance tests.

use super::{EncodeSet, encode};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies the seven pinned UTF-8 WPT expectations.
#[test]
fn pinned_utf8_fixture_success() {
    for (input, expected) in [
        ("†", "%E2%80%A0"),
        ("\u{000e}A", "%0EA"),
        ("‾\\", "%E2%80%BE\\"),
        ("\u{e5e5}", "%EE%97%A5"),
        ("−", "%E2%88%92"),
        ("¢", "%C2%A2"),
        ("á|", "%C3%A1|"),
    ] {
        assert_eq!(encode(input, EncodeSet::C0Control), expected);
    }
}
