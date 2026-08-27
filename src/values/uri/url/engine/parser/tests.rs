//! Tests for private parser and resolver behavior.

use super::{Validation, parse, wpt_vectors::VECTORS};
use crate::values::uri::url::engine::{
    host,
    record::{Path, Record},
    serializer,
};

#[test]
/// Verifies absolute special parsing yields one canonical private serialization.
fn absolute_special_url_success() {
    let outcome = parse("http://user:pass@example.test:80/a/../b?x=#", None);
    assert!(matches!(
        outcome.as_ref().map(|outcome| serializer::serialize(&outcome.record)),
        Ok(value) if value == "http://user:pass@example.test/b?x=#"
    ));
    assert!(outcome.is_ok_and(|outcome| outcome.record.port.is_none()));
}

#[test]
/// Verifies relative parsing inherits the authority and does not mutate its base.
fn relative_resolution_success() {
    let base = parse("https://example.test/a/b?old", None);
    let result = match &base {
        Ok(base) => parse("../c?new", Some(&base.record)),
        Err(_) => panic!(),
    };
    assert!(matches!(
        result.as_ref().map(|outcome| serializer::serialize(&outcome.record)),
        Ok(value) if value == "https://example.test/c?new"
    ));
    assert!(matches!(
        base.as_ref().map(|outcome| &outcome.record.path),
        Ok(Path::Segments(segments)) if segments == &["".to_owned(), "a".to_owned(), "b".to_owned()]
    ));
}

#[test]
/// Verifies special backslashes are retained as typed nonfatal validation.
fn special_backslash_validation_success() {
    let outcome = parse("http:\\example.test\\path", None);
    assert!(
        outcome.is_ok_and(|outcome| outcome.validations.contains(&Validation::SpecialBackslash))
    );
}

#[test]
/// Verifies opaque bases reject non-fragment relative paths.
fn opaque_base_relative_path_error() {
    let base = parse("mailto:person@example.test", None);
    assert!(base.is_ok());
    assert!(base.is_ok_and(|base| parse("next", Some(&base.record)).is_err()));
}

#[test]
/// Verifies pinned WPT canonical serializations remain stable on a second parse.
fn pinned_urltestdata_parse_serialize_idempotence_success() {
    for (index, vector) in VECTORS
        .iter()
        .enumerate()
        .filter(|(_, vector)| !vector.failure)
    {
        let base = match vector.base {
            Some(value) => match parse(value, None) {
                Ok(outcome) => Some(outcome.record),
                Err(_) => panic!("invalid generated base at row {index}"),
            },
            None => None,
        };
        let first = match parse(vector.input, base.as_ref()) {
            Ok(outcome) => outcome,
            Err(_) => panic!("unexpected failure at row {index}"),
        };
        let serialized = serializer::serialize(&first.record);
        let second = match parse(&serialized, None) {
            Ok(outcome) => outcome,
            Err(_) => panic!("canonical parse failed at row {index}"),
        };
        assert_eq!(
            serializer::serialize(&second.record),
            serialized,
            "row {index}"
        );
    }
}

#[test]
/// Verifies the RFC 3986 normal and abnormal resolution examples under WHATWG parsing.
fn rfc_3986_resolution_vectors_success() {
    let base = match parse("http://a/b/c/d;p?q", None) {
        Ok(outcome) => outcome.record,
        Err(_) => panic!(),
    };
    let vectors = [
        ("g:h", "g:h"),
        ("g", "http://a/b/c/g"),
        ("./g", "http://a/b/c/g"),
        ("g/", "http://a/b/c/g/"),
        ("/g", "http://a/g"),
        ("//g", "http://g/"),
        ("?y", "http://a/b/c/d;p?y"),
        ("g?y", "http://a/b/c/g?y"),
        ("#s", "http://a/b/c/d;p?q#s"),
        ("g#s", "http://a/b/c/g#s"),
        ("g?y#s", "http://a/b/c/g?y#s"),
        (";x", "http://a/b/c/;x"),
        ("g;x", "http://a/b/c/g;x"),
        ("", "http://a/b/c/d;p?q"),
        (".", "http://a/b/c/"),
        ("./", "http://a/b/c/"),
        ("..", "http://a/b/"),
        ("../", "http://a/b/"),
        ("../g", "http://a/b/g"),
        ("../../g", "http://a/g"),
        ("../../.", "http://a/"),
        ("../../../g", "http://a/g"),
        ("../../../../g", "http://a/g"),
        ("/./g", "http://a/g"),
        ("/../g", "http://a/g"),
        ("g.", "http://a/b/c/g."),
        ("g..", "http://a/b/c/g.."),
        ("./../g", "http://a/b/g"),
        ("./g/.", "http://a/b/c/g/"),
        ("g/./h", "http://a/b/c/g/h"),
        ("g/../h", "http://a/b/c/h"),
        ("g;x=1/./y", "http://a/b/c/g;x=1/y"),
        ("g;x=1/../y", "http://a/b/c/y"),
        ("g?y/./x", "http://a/b/c/g?y/./x"),
        ("g?y/../x", "http://a/b/c/g?y/../x"),
        ("g#s/./x", "http://a/b/c/g#s/./x"),
        ("g#s/../x", "http://a/b/c/g#s/../x"),
        ("http:g", "http://a/b/c/g"),
        ("http:", "http://a/b/c/d;p?q"),
    ];
    for (reference, expected) in vectors {
        let resolved = match parse(reference, Some(&base)) {
            Ok(outcome) => outcome,
            Err(_) => panic!("failed reference {reference}"),
        };
        assert_eq!(
            serializer::serialize(&resolved.record),
            expected,
            "{reference}"
        );
    }
}

#[test]
/// Verifies long malformed and expansion-heavy inputs return without panicking.
fn long_adversarial_inputs_do_not_panic_success() {
    let long_authority = format!("http://{}", "a".repeat(32_768));
    let long_percent = format!("custom:{}", "%25".repeat(32_768));
    let long_path = format!("https://example.test/{}", "../".repeat(16_384));
    let results = [
        parse(&long_authority, None),
        parse(&long_percent, None),
        parse(&long_path, None),
    ];
    assert_eq!(results.len(), 3);
}

#[test]
/// Verifies every pinned URL WPT object row through the same private parser.
fn pinned_urltestdata_vectors_success() {
    for (index, vector) in VECTORS.iter().enumerate() {
        let base = match vector.base {
            Some(value) => match parse(value, None) {
                Ok(outcome) => Some(outcome.record),
                Err(_) => panic!("invalid generated base at row {index}"),
            },
            None => None,
        };
        let result = parse(vector.input, base.as_ref());
        assert_eq!(
            result.is_err(),
            vector.failure,
            "unexpected outcome at row {index}"
        );
        if let Ok(outcome) = result {
            let projection = projection(&outcome.record);
            assert_eq!(
                Some(projection.href.as_str()),
                vector.href,
                "href row {index}"
            );
            assert_eq!(
                Some(projection.protocol.as_str()),
                vector.protocol,
                "protocol row {index}"
            );
            assert_eq!(
                Some(projection.username.as_str()),
                vector.username,
                "username row {index}"
            );
            assert_eq!(
                Some(projection.password.as_str()),
                vector.password,
                "password row {index}"
            );
            assert_eq!(
                Some(projection.host.as_str()),
                vector.host,
                "host row {index}"
            );
            assert_eq!(
                Some(projection.hostname.as_str()),
                vector.hostname,
                "hostname row {index}"
            );
            assert_eq!(
                Some(projection.port.as_str()),
                vector.port,
                "port row {index}"
            );
            assert_eq!(
                Some(projection.pathname.as_str()),
                vector.pathname,
                "pathname row {index}"
            );
            assert_eq!(
                Some(projection.search.as_str()),
                vector.search,
                "search row {index}"
            );
            assert_eq!(
                Some(projection.hash.as_str()),
                vector.hash,
                "hash row {index}"
            );
        }
    }
}

struct Projection {
    href: String,
    protocol: String,
    username: String,
    password: String,
    host: String,
    hostname: String,
    port: String,
    pathname: String,
    search: String,
    hash: String,
}

fn projection(record: &Record) -> Projection {
    let hostname = record
        .host
        .as_ref()
        .map_or_else(String::new, host::serialize);
    let port = record
        .port
        .map_or_else(String::new, |value| value.to_string());
    let host = if port.is_empty() {
        hostname.clone()
    } else {
        format!("{hostname}:{port}")
    };
    let pathname = match &record.path {
        Path::Segments(segments) => {
            let value = segments.join("/");
            if !record.special && !record.has_authority {
                value.strip_prefix("/.").unwrap_or(&value).to_owned()
            } else {
                value
            }
        }
        Path::Opaque(value) => value.clone(),
    };
    Projection {
        href: serializer::serialize(record),
        protocol: format!("{}:", record.scheme),
        username: record.username.clone(),
        password: record.password.clone().unwrap_or_default(),
        host,
        hostname,
        port,
        pathname,
        search: record.query.as_ref().map_or_else(String::new, |value| {
            if value.is_empty() {
                String::new()
            } else {
                format!("?{value}")
            }
        }),
        hash: record.fragment.as_ref().map_or_else(String::new, |value| {
            if value.is_empty() {
                String::new()
            } else {
                format!("#{value}")
            }
        }),
    }
}
