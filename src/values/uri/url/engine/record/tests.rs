//! Tests for private canonical URL record invariants.

use super::{Authority, Error, Path, Record};
use crate::values::uri::url::engine::host::Host;

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
