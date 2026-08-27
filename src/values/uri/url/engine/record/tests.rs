//! Tests for private canonical URL record invariants.

use super::{Authority, Error, Path, Record};
use crate::values::uri::url::engine::{host::Host, parser, serializer};

#[test]
/// Verifies special records reject opaque hosts.
fn special_record_rejects_opaque_host_error() {
    assert!(matches!(
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("example".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new()]),
            None,
            None,
        ),
        Err(Error::Invalid)
    ));
}

#[test]
/// Verifies an opaque path cannot coexist with an authority.
fn opaque_path_with_authority_error() {
    assert!(matches!(
        Record::new(
            "mailto".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Opaque("person@example.test".to_owned()),
            None,
            None,
        ),
        Err(Error::Invalid)
    ));
}

#[test]
/// Verifies empty passwords normalize to the canonical absent password state.
fn empty_password_normalizes_success() {
    let record = Record::new(
        "https".to_owned(),
        Some(Authority::new(
            "user".to_owned(),
            Some(String::new()),
            Host::Domain("example.test".to_owned()),
            Some(443),
        )),
        Path::Segments(vec![String::new(), String::new()]),
        Some(String::new()),
        Some(String::new()),
    );
    assert!(record.is_ok_and(|record| {
        record.password().is_none()
            && record.port().is_none()
            && record.query() == Some("")
            && record.fragment() == Some("")
    }));
}

#[test]
/// Verifies absent and empty hosts remain distinct canonical representations.
fn absent_and_empty_host_are_distinct_success() {
    let absent = Record::new(
        "mailto".to_owned(),
        None,
        Path::Opaque("person@example.test".to_owned()),
        None,
        None,
    );
    let empty = Record::new(
        "file".to_owned(),
        Some(Authority::new(String::new(), None, Host::Empty, None)),
        Path::Segments(vec![String::new(), String::new()]),
        None,
        None,
    );
    assert!(absent.is_ok_and(|record| record.host().is_none()));
    assert!(empty.is_ok_and(|record| matches!(record.host(), Some(Host::Empty))));
}

#[test]
/// Verifies constructor-only state cannot represent invalid special or file records.
fn canonical_record_invariants_error() {
    let rooted = Path::Segments(vec![String::new(), "path".to_owned()]);
    assert!(Record::new("HTTP".to_owned(), None, rooted.clone(), None, None).is_err());
    assert!(Record::new("http".to_owned(), None, rooted.clone(), None, None).is_err());
    assert!(
        Record::new(
            "http".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            rooted.clone(),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Opaque("path".to_owned()),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "file".to_owned(),
            Some(Authority::new("user".to_owned(), None, Host::Empty, None,)),
            rooted.clone(),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "file".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, Some(80))),
            rooted,
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("host".to_owned()),
                None,
            )),
            Path::Segments(vec!["not-rooted".to_owned()]),
            None,
            None,
        )
        .is_err()
    );
}

#[test]
/// Verifies later canonical constraints exclude states that would change on parsing.
fn serialize_parse_stable_record_states_error() {
    assert!(
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new()]),
            None,
            None,
        )
        .is_err()
    );
    for host in [
        Host::Domain("example.test".to_owned()),
        Host::Ipv4(0x7f000001),
    ] {
        assert!(
            Record::new(
                "custom".to_owned(),
                Some(Authority::new(String::new(), None, host, None)),
                Path::Segments(Vec::new()),
                None,
                None,
            )
            .is_err()
        );
    }
    assert!(
        Record::new(
            "custom".to_owned(),
            Some(Authority::new("user".to_owned(), None, Host::Empty, None)),
            Path::Segments(Vec::new()),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, Some(1))),
            Path::Segments(Vec::new()),
            None,
            None,
        )
        .is_err()
    );
    for path in [
        Path::Segments(Vec::new()),
        Path::Segments(vec!["path".to_owned()]),
    ] {
        assert!(Record::new("custom".to_owned(), None, path, None, None).is_err());
    }
}

#[test]
/// Verifies constructor input is already in each component's canonical lexical form.
fn noncanonical_record_components_error() {
    let rooted = Path::Segments(vec![String::new(), "path".to_owned()]);
    for host in [
        Host::Domain(String::new()),
        Host::Domain("EXAMPLE.TEST".to_owned()),
        Host::Domain("127.0.0.1".to_owned()),
    ] {
        assert!(
            Record::new(
                "http".to_owned(),
                Some(Authority::new(String::new(), None, host, None)),
                rooted.clone(),
                None,
                None,
            )
            .is_err()
        );
    }
    for host in [
        Host::Opaque(String::new()),
        Host::Opaque("forbidden/path".to_owned()),
    ] {
        assert!(
            Record::new(
                "custom".to_owned(),
                Some(Authority::new(String::new(), None, host, None)),
                Path::Segments(Vec::new()),
                None,
                None,
            )
            .is_err()
        );
    }
    for authority in [
        Authority::new(
            "user@name".to_owned(),
            None,
            Host::Domain("example.test".to_owned()),
            None,
        ),
        Authority::new(
            String::new(),
            Some("pass/word".to_owned()),
            Host::Domain("example.test".to_owned()),
            None,
        ),
    ] {
        assert!(
            Record::new(
                "http".to_owned(),
                Some(authority),
                rooted.clone(),
                None,
                None
            )
            .is_err()
        );
    }
    for path in [
        Path::Segments(vec![String::new(), "a/b".to_owned()]),
        Path::Segments(vec![String::new(), "has space".to_owned()]),
        Path::Opaque("opaque ".to_owned()),
    ] {
        assert!(Record::new("custom".to_owned(), None, path, None, None).is_err());
    }
    assert!(
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque("opaque".to_owned()),
            Some("raw#hash".to_owned()),
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque("opaque".to_owned()),
            None,
            Some("raw\"quote".to_owned()),
        )
        .is_err()
    );
}

#[test]
/// Verifies paths that would change during private serialization and parsing are rejected.
fn unstable_record_paths_error() {
    let special_authority = || {
        Some(Authority::new(
            String::new(),
            None,
            Host::Domain("example.test".to_owned()),
            None,
        ))
    };
    assert!(
        Record::new(
            "http".to_owned(),
            special_authority(),
            Path::Segments(vec![String::new(), ".".to_owned(), "b".to_owned()]),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![
                String::new(),
                "a".to_owned(),
                "..".to_owned(),
                "b".to_owned(),
            ]),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![String::new(), "%2e".to_owned(), "b".to_owned()]),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![String::new(), String::new(), "x".to_owned()]),
            None,
            None,
        )
        .is_err()
    );
    assert!(
        Record::new(
            "file".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(vec![String::new(), "C|".to_owned(), "x".to_owned()]),
            None,
            None,
        )
        .is_err()
    );
}

#[test]
/// Verifies segmented paths reject the remaining serialization-unstable forms.
fn single_empty_and_special_backslash_segment_paths_error() {
    for record in [
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![String::new()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("host".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new()]),
            None,
            None,
        ),
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new(), "a\\b".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "file".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(vec![String::new(), "a\\b".to_owned()]),
            None,
            None,
        ),
    ] {
        assert!(record.is_err());
    }
}

#[test]
/// Verifies opaque paths cannot change parser state or component boundaries.
fn unstable_opaque_paths_error() {
    for path in ["/a", "//a", "a?b", "a#b", "a ", "a\u{0001}"] {
        assert!(
            Record::new(
                "custom".to_owned(),
                None,
                Path::Opaque(path.to_owned()),
                None,
                None,
            )
            .is_err()
        );
    }
}

#[test]
/// Verifies the bounded private record invariant across canonical URL shapes.
fn canonical_record_shapes_permanent_invariant_success() {
    let records = [
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new(), "C|".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "file".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(vec![String::new(), "C:".to_owned(), "x".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("host".to_owned()),
                None,
            )),
            Path::Segments(Vec::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(vec![String::new(), String::new()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![
                String::new(),
                ".".to_owned(),
                String::new(),
                "x".to_owned(),
            ]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![String::new(), "a\\b".to_owned(), "%5C".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque(String::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque("a b".to_owned()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque("a%3F%23%zz\\b".to_owned()),
            None,
            None,
        ),
    ];
    for record in records {
        let record = match record {
            Ok(record) => record,
            Err(_) => panic!(),
        };
        let serialized = serializer::serialize(&record);
        let reparsed = match parser::parse(&serialized, None) {
            Ok(outcome) => outcome.record,
            Err(_) => panic!("{serialized}"),
        };
        assert!(reparsed == record, "{serialized}");
    }
}

#[test]
/// Verifies accepted constructor examples are stable after private serialization and parsing.
fn canonical_record_component_examples_round_trip_success() {
    let records = [
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                "user%20name".to_owned(),
                Some("pass%20word".to_owned()),
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new(), "path%20part".to_owned()]),
            Some("query=%2F".to_owned()),
            Some("fragment%23".to_owned()),
        ),
        Record::new(
            "file".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(vec![String::new(), "C:".to_owned(), "path".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Ipv4(0x7f000001),
                None,
            )),
            Path::Segments(vec![String::new(), String::new()]),
            None,
            None,
        ),
        Record::new(
            "http".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Ipv6([0, 0, 0, 0, 0, 0, 0, 1]),
                None,
            )),
            Path::Segments(vec![String::new(), String::new()]),
            None,
            None,
        ),
        Record::new(
            "file".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Domain("example.test".to_owned()),
                None,
            )),
            Path::Segments(vec![String::new(), "share".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("opaque-host".to_owned()),
                None,
            )),
            Path::Segments(Vec::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(Vec::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Ipv6([0, 0, 0, 0, 0, 0, 0, 1]),
                None,
            )),
            Path::Segments(Vec::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![String::new(), "rooted".to_owned()]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Segments(vec![
                String::new(),
                ".".to_owned(),
                String::new(),
                "x".to_owned(),
            ]),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            Some(Authority::new(
                String::new(),
                None,
                Host::Opaque("bad%zz".to_owned()),
                None,
            )),
            Path::Segments(Vec::new()),
            None,
            None,
        ),
        Record::new(
            "custom".to_owned(),
            None,
            Path::Opaque("opaque path".to_owned()),
            None,
            None,
        ),
    ];
    for record in records {
        let record = match record {
            Ok(record) => record,
            Err(_) => panic!(),
        };
        let serialized = serializer::serialize(&record);
        let reparsed = match parser::parse(&serialized, None) {
            Ok(outcome) => outcome.record,
            Err(_) => panic!("{serialized}"),
        };
        assert!(reparsed == record, "{serialized}");
    }
}
