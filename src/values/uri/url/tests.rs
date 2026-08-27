//! Public URL contract tests.

use super::URL;
use crate::{
    error::{Audience, Kind},
    values::Value,
};
use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher};

fn canonical_value<T: Value<ValueType = str>>(value: &T) -> &str {
    value.value()
}

fn hash_of(value: &URL) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
/// Parses a canonical URL and exposes the frozen borrowed component contract.
fn try_new_and_accessors_success() {
    let result = URL::try_new("https://user:pass@example.test:8443/path?x=1&x=&=empty#fragment");
    assert!(result.is_ok());
    let url = match result {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };

    assert_eq!(
        url.value(),
        "https://user:pass@example.test:8443/path?x=1&x=&=empty#fragment"
    );
    assert_eq!(canonical_value(&url), url.value());
    assert_eq!(url.scheme(), "https");
    assert_eq!(url.username(), "user");
    assert_eq!(url.password(), Some("pass"));
    assert_eq!(url.host(), Some("example.test"));
    assert_eq!(url.port(), Some(8443));
    assert_eq!(url.path(), "/path");
    assert_eq!(url.query(), Some("x=1&x=&=empty"));
    assert_eq!(
        url.query_pairs(),
        [
            ("x".to_owned(), "1".to_owned()),
            ("x".to_owned(), "".to_owned()),
            ("".to_owned(), "empty".to_owned())
        ]
    );
    assert_eq!(url.fragment(), Some("fragment"));
    assert!(!url.has_opaque_path());
}

#[test]
/// Preserves absent components independently from present-empty delimiters.
fn absent_and_present_empty_components_success() {
    let absent = URL::try_new("https://example.test/");
    assert!(absent.is_ok());
    let absent = match absent {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    assert_eq!(absent.password(), None);
    assert_eq!(absent.port(), None);
    assert_eq!(absent.query(), None);
    assert_eq!(absent.fragment(), None);

    let present_empty = URL::try_new("https://example.test/?#");
    assert!(present_empty.is_ok());
    let present_empty = match present_empty {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    assert_eq!(present_empty.query(), Some(""));
    assert_eq!(present_empty.fragment(), Some(""));
    assert!(present_empty.query_pairs().is_empty());
}

#[test]
/// Resolves through the private parser without mutating the canonical base.
fn try_resolve_is_immutable_success() {
    let base = URL::try_new("https://example.test/a/b?base=1#old");
    assert!(base.is_ok());
    let base = match base {
        Ok(url) => url,
        Err(error) => panic!("unexpected base parse failure: {error}"),
    };
    let resolved = base.try_resolve("../next?x=1#new");
    assert!(resolved.is_ok());
    let resolved = match resolved {
        Ok(url) => url,
        Err(error) => panic!("unexpected resolution failure: {error}"),
    };
    assert_eq!(resolved.value(), "https://example.test/next?x=1#new");
    assert_eq!(base.value(), "https://example.test/a/b?base=1#old");
}

#[test]
/// Reconstructs one canonical URL while preserving the original immutable value.
fn mutator_chaining_and_last_setter_success() {
    let original = URL::try_new("https://old@example.test/old?old=1#old");
    assert!(original.is_ok());
    let original = match original {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    let mutated = original
        .mutator()
        .scheme("HTTP")
        .username("first")
        .username("new@user")
        .password(Some("new:pass"))
        .host(Some("new.example.test"))
        .port(Some(80))
        .path("/next?not-query")
        .query(Some("a=1&a=&=z+plus"))
        .fragment(Some("new fragment"))
        .try_mutate();
    assert!(mutated.is_ok());
    let mutated = match mutated {
        Ok(url) => url,
        Err(error) => panic!("unexpected mutation failure: {error}"),
    };
    assert_eq!(
        mutated.value(),
        "http://new%40user:new%3Apass@new.example.test/next%3Fnot-query?a=1&a=&=z+plus#new%20fragment"
    );
    assert_eq!(mutated.port(), None);
    assert_eq!(
        mutated.query_pairs()[2],
        ("".to_owned(), "z plus".to_owned())
    );
    assert_eq!(original.value(), "https://old@example.test/old?old=1#old");
}

#[test]
/// Rejects component replacements that would inject a new authority or special host state.
fn mutator_rejects_structural_component_injection_error() {
    let special = URL::try_new("https://example.test/path");
    assert!(special.is_ok());
    let special = match special {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    let missing_special_host = special.mutator().host(None).try_mutate();
    assert!(missing_special_host.is_err());
    let missing_special_host = match missing_special_host {
        Ok(_) => panic!("special host removal unexpectedly succeeded"),
        Err(error) => error,
    };
    assert_eq!(missing_special_host.message(), "URL mutation failed.");

    let opaque = URL::try_new("custom:value");
    assert!(opaque.is_ok());
    let opaque = match opaque {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    let authority_injection = opaque.mutator().path("//example.test").try_mutate();
    assert!(authority_injection.is_err());
}

#[test]
/// Uses complete canonical values for equality and hash consistency.
fn equality_and_hash_success() {
    let first = URL::try_new("HTTP://example.test/path?x=1#part");
    let second = URL::try_new("http://example.test/path?x=1#part");
    assert!(first.is_ok());
    assert!(second.is_ok());
    let first = match first {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    let second = match second {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    assert_eq!(first, second);
    assert_eq!(hash_of(&first), hash_of(&second));
}

#[test]
/// Redacts automatic URL diagnostics while retaining explicit sensitive access.
fn debug_and_errors_are_redacted_success() {
    let sensitive =
        URL::try_new("https://canaryuser:canarypass@example.test/?canaryquery#canaryfragment");
    assert!(sensitive.is_ok());
    let sensitive = match sensitive {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };
    assert_eq!(format!("{sensitive:?}"), "URL([REDACTED])");
    assert!(sensitive.value().contains("canaryuser"));

    let parse_error =
        URL::try_new("https://canaryuser:canarypass@[::zzz]/?canaryquery#canaryfragment");
    assert!(parse_error.is_err());
    let parse_error = match parse_error {
        Ok(_) => panic!("invalid URL unexpectedly parsed"),
        Err(error) => error,
    };
    assert_eq!(parse_error.kind(), Kind::InvalidInput);
    assert_eq!(parse_error.audience(), Audience::User);
    assert_eq!(parse_error.message(), "URL parsing failed.");
    for canary in ["canaryuser", "canarypass", "canaryquery", "canaryfragment"] {
        assert!(!parse_error.message().contains(canary));
    }

    let resolution_error =
        sensitive.try_resolve("https://canaryuser:canarypass@[::zzz]/?canaryquery#canaryfragment");
    assert!(resolution_error.is_err());
    let resolution_error = match resolution_error {
        Ok(_) => panic!("invalid reference unexpectedly resolved"),
        Err(error) => error,
    };
    assert_eq!(resolution_error.kind(), Kind::InvalidInput);
    assert_eq!(resolution_error.audience(), Audience::User);
    assert_eq!(resolution_error.message(), "URL resolution failed.");
    for canary in ["canaryuser", "canarypass", "canaryquery", "canaryfragment"] {
        assert!(!resolution_error.message().contains(canary));
    }

    let mutation_error = sensitive
        .mutator()
        .host(Some("canaryuser/path"))
        .try_mutate();
    assert!(mutation_error.is_err());
    let mutation_error = match mutation_error {
        Ok(_) => panic!("invalid mutation unexpectedly succeeded"),
        Err(error) => error,
    };
    assert_eq!(mutation_error.message(), "URL mutation failed.");
    assert!(!mutation_error.message().contains("canaryuser"));
}
