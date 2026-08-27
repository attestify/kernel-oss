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
    assert!(outcome.is_ok_and(|outcome| outcome.record.port().is_none()));
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
        base.as_ref().map(|outcome| outcome.record.path()),
        Ok(Path::Segments(segments)) if segments == &["".to_owned(), "a".to_owned(), "b".to_owned()]
    ));
}

#[test]
/// Verifies special backslashes are retained as typed nonfatal validation.
fn special_backslash_validation_success() {
    let outcome = parse("http:\\example.test\\path", None);
    assert!(outcome.is_ok_and(|outcome| {
        outcome
            .validations
            .contains(&Validation::InvalidReverseSolidus)
    }));
}

#[test]
/// Verifies special authority ignore-slashes consumes arbitrary slash runs.
fn special_authority_ignore_slashes_success() {
    let vectors = [
        ("http:///foo", "http://foo/"),
        ("http:////foo", "http://foo/"),
        ("https://///example.com/path", "https://example.com/path"),
        ("ftp:////host/path", "ftp://host/path"),
        ("http:\\\\\\foo", "http://foo/"),
        ("file:////server/share", "file:////server/share"),
    ];
    for (input, expected) in vectors {
        let outcome = parse(input, None);
        assert!(matches!(
            outcome.as_ref().map(|outcome| serializer::serialize(&outcome.record)),
            Ok(value) if value == expected
        ));
    }
}

#[test]
/// Verifies file authority and leading-path-slash forms remain distinct.
fn file_authority_and_path_slashes_success() {
    let server = parse("file://server/share", None);
    assert!(matches!(
        server.as_ref(),
        Ok(outcome)
            if outcome.record.host().is_some_and(|host| host::serialize(host) == "server")
                && matches!(outcome.record.path(), Path::Segments(segments) if segments.join("/") == "/share")
    ));
    let empty_host = parse("file:///server/share", None);
    assert!(matches!(
        empty_host.as_ref(),
        Ok(outcome)
            if outcome.record.host().is_some_and(|host| host::serialize(host).is_empty())
                && matches!(outcome.record.path(), Path::Segments(segments) if segments.join("/") == "/server/share")
    ));
    let doubled_path = parse("file:////server/share", None);
    assert!(matches!(
        doubled_path.as_ref().map(|outcome| serializer::serialize(&outcome.record)),
        Ok(value) if value == "file:////server/share"
    ));
}

#[test]
/// Verifies authority, ports, delimiters, and opaque-base transitions remain canonical.
fn focused_state_transitions_success() {
    let credentials = parse("https://a@b@example.test:443/path?x=#", None);
    assert!(credentials.is_ok_and(|outcome| {
        serializer::serialize(&outcome.record) == "https://a%40b@example.test/path?x=#"
            && outcome
                .validations
                .contains(&Validation::InvalidCredentials)
            && outcome.record.port().is_none()
            && outcome.record.query() == Some("x=")
            && outcome.record.fragment() == Some("")
    }));
    assert!(parse("https://example.test:65536/", None).is_err());
    let opaque_base = parse("mailto:person@example.test", None);
    assert!(opaque_base.is_ok_and(|base| {
        parse("#fragment", Some(&base.record)).is_ok()
            && parse("relative", Some(&base.record)).is_err()
    }));
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
        assert!(second.record == first.record, "record row {index}");
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
        ("g;x?y#s", "http://a/b/c/g;x?y#s"),
        (";x", "http://a/b/c/;x"),
        ("g;x", "http://a/b/c/g;x"),
        ("", "http://a/b/c/d;p?q"),
        (".", "http://a/b/c/"),
        ("./", "http://a/b/c/"),
        ("..", "http://a/b/"),
        ("../", "http://a/b/"),
        ("../g", "http://a/b/g"),
        ("../../g", "http://a/g"),
        ("../..", "http://a/"),
        ("../../", "http://a/"),
        ("../../../g", "http://a/g"),
        ("../../../../g", "http://a/g"),
        ("/./g", "http://a/g"),
        ("/../g", "http://a/g"),
        ("g.", "http://a/b/c/g."),
        ("g..", "http://a/b/c/g.."),
        (".g", "http://a/b/c/.g"),
        ("..g", "http://a/b/c/..g"),
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
/// Verifies structured long inputs exercise parser states without recursion or panics.
fn structured_adversarial_inputs_success() {
    let corpus = [
        format!("http://{}", "a".repeat(32_768)),
        format!("custom:{}", "%25".repeat(32_768)),
        format!("https://example.test/{}", "../".repeat(16_384)),
        format!("https://user:{}@example.test/", "p".repeat(16_384)),
        format!("https://example.test/?{}", "%25&".repeat(16_384)),
        format!("file:///C:/{}", "a/".repeat(16_384)),
    ];
    for input in corpus {
        if let Ok(outcome) = parse(&input, None) {
            let serialized = serializer::serialize(&outcome.record);
            let reparsed = parse(&serialized, None);
            assert!(reparsed.is_ok());
        }
    }
}

#[test]
/// Verifies component validation separates nonfatal malformed input from fatal parsing.
fn component_validation_conditions_success() {
    let outcome = parse("custom://user%ZZ@example/%ZZ?%ZZ#%ZZ", None);
    assert!(outcome.is_ok_and(|outcome| {
        outcome
            .validations
            .contains(&Validation::InvalidCredentials)
            && outcome.validations.contains(&Validation::InvalidUrlUnit)
    }));
    let special = parse("http:example.test", None);
    assert!(special.is_ok_and(|outcome| {
        outcome
            .validations
            .contains(&Validation::SpecialSchemeMissingFollowingSolidus)
    }));
    let opaque = parse("custom:opaque%ZZ\u{000E}tail", None);
    assert!(opaque.is_ok_and(|outcome| {
        outcome.validations.contains(&Validation::InvalidUrlUnit)
            && serializer::serialize(&outcome.record).contains("%ZZ%0E")
    }));
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
    let hostname = record.host().map_or_else(String::new, host::serialize);
    let port = record
        .port()
        .map_or_else(String::new, |value| value.to_string());
    let host = if port.is_empty() {
        hostname.clone()
    } else {
        format!("{hostname}:{port}")
    };
    let pathname = match record.path() {
        Path::Segments(segments) => {
            let value = segments.join("/");
            if !record.is_special() && !record.has_authority() {
                value.strip_prefix("/.").unwrap_or(&value).to_owned()
            } else {
                value
            }
        }
        Path::Opaque(value) => value.clone(),
    };
    Projection {
        href: serializer::serialize(record),
        protocol: format!("{}:", record.scheme()),
        username: record.username().to_owned(),
        password: record.password().unwrap_or_default().to_owned(),
        host,
        hostname,
        port,
        pathname,
        search: record.query().map_or_else(String::new, |value| {
            if value.is_empty() {
                String::new()
            } else {
                format!("?{value}")
            }
        }),
        hash: record.fragment().map_or_else(String::new, |value| {
            if value.is_empty() {
                String::new()
            } else {
                format!("#{value}")
            }
        }),
    }
}
