//! Tests for private canonical URL serialization.

use super::serialize;
use crate::values::uri::url::engine::{
    host::Host,
    record::{Authority, Path, Record},
};

#[test]
/// Verifies serialization preserves empty query and fragment delimiter presence.
fn empty_query_and_fragment_success() {
    let record = Record::new(
        "http".to_owned(),
        Some(Authority::new(
            String::new(),
            None,
            Host::Domain("example.test".to_owned()),
            None,
        )),
        Path::Segments(vec![String::new(), String::new()]),
        Some(String::new()),
        Some(String::new()),
    );
    assert!(matches!(
        record.as_ref().map(serialize),
        Ok(value) if value == "http://example.test/?#"
    ));
}
