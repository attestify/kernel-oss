//! Tests for private canonical URL record invariants.

use super::{Error, Path, Record};
use crate::values::uri::url::engine::host::Host;

#[test]
/// Verifies special records reject opaque hosts.
fn special_record_rejects_opaque_host_error() {
    assert!(matches!(
        Record::new(
            "http".to_owned(),
            String::new(),
            None,
            Some(Host::Opaque("example".to_owned())),
            None,
            Path::Segments(vec![String::new()]),
            None,
            None,
            true,
            true,
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
            String::new(),
            None,
            None,
            None,
            Path::Opaque("person@example.test".to_owned()),
            None,
            None,
            true,
            false,
        ),
        Err(Error::Invalid)
    ));
}
