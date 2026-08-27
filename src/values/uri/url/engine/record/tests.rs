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
