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
/// Keeps host replacement bounded to one canonical host component.
fn mutator_host_boundary_success() {
    let base =
        URL::try_new("https://user%40name:pass%3Aword@example.test/a%2Fb?stable=%25#stable%23");
    assert!(base.is_ok());
    let base = match base {
        Ok(url) => url,
        Err(error) => panic!("unexpected URL parse failure: {error}"),
    };

    for value in ["attacker.test:4444", "[::1]:4444"] {
        let result = base.mutator().host(Some(value)).try_mutate();
        assert!(result.is_err());
        let error = match result {
            Ok(_) => panic!("host carrying a port unexpectedly succeeded"),
            Err(error) => error,
        };
        assert_eq!(error.message(), "URL mutation failed.");
    }
    for value in ["host\tname", "host\nname", "host\rname"] {
        assert!(base.mutator().host(Some(value)).try_mutate().is_err());
    }
    for value in [
        "user@example.test",
        "example.test/path",
        "example.test?query",
        "example.test#fragment",
    ] {
        assert!(base.mutator().host(Some(value)).try_mutate().is_err());
    }

    let domain = base.mutator().host(Some("attacker.test")).try_mutate();
    assert!(domain.is_ok());
    let domain = match domain {
        Ok(url) => url,
        Err(error) => panic!("valid domain replacement failed: {error}"),
    };
    assert_eq!(domain.host(), Some("attacker.test"));
    assert_eq!(domain.username(), "user%40name");
    assert_eq!(domain.password(), Some("pass%3Aword"));
    assert_eq!(domain.path(), "/a%2Fb");
    assert_eq!(domain.query(), Some("stable=%25"));
    assert_eq!(domain.fragment(), Some("stable%23"));
    assert_eq!(
        domain.value(),
        "https://user%40name:pass%3Aword@attacker.test/a%2Fb?stable=%25#stable%23"
    );

    let unicode = base.mutator().host(Some("bücher.example")).try_mutate();
    assert!(unicode.is_ok());
    let unicode = match unicode {
        Ok(url) => url,
        Err(error) => panic!("valid IDNA replacement failed: {error}"),
    };
    assert_eq!(unicode.host(), Some("xn--bcher-kva.example"));
    let ipv4 = base.mutator().host(Some("127.0.0.1")).try_mutate();
    assert!(ipv4.is_ok());
    let ipv4 = match ipv4 {
        Ok(url) => url,
        Err(error) => panic!("valid IPv4 replacement failed: {error}"),
    };
    assert_eq!(ipv4.host(), Some("127.0.0.1"));
    let ipv6 = base.mutator().host(Some("[::1]")).try_mutate();
    assert!(ipv6.is_ok());
    let ipv6 = match ipv6 {
        Ok(url) => url,
        Err(error) => panic!("valid IPv6 replacement failed: {error}"),
    };
    assert_eq!(ipv6.host(), Some("[::1]"));

    let with_port = base
        .mutator()
        .host(Some("attacker.test"))
        .port(Some(4444))
        .try_mutate();
    assert!(with_port.is_ok());
    let with_port = match with_port {
        Ok(url) => url,
        Err(error) => panic!("independent host and port mutation failed: {error}"),
    };
    assert_eq!(with_port.host(), Some("attacker.test"));
    assert_eq!(with_port.port(), Some(4444));

    let first = base
        .mutator()
        .host(Some("attacker.test"))
        .port(Some(4444))
        .query(Some("x=1"))
        .try_mutate();
    let second = base
        .mutator()
        .query(Some("x=1"))
        .port(Some(4444))
        .host(Some("attacker.test"))
        .try_mutate();
    assert!(first.is_ok());
    assert!(second.is_ok());
    let first = match first {
        Ok(url) => url,
        Err(error) => panic!("first setter order failed: {error}"),
    };
    let second = match second {
        Ok(url) => url,
        Err(error) => panic!("second setter order failed: {error}"),
    };
    assert_eq!(first, second);
}

#[test]
/// Preserves the permitted empty file host while canonicalizing it through mutation.
fn mutator_permitted_empty_file_host_success() {
    let file = URL::try_new("file:///a");
    assert!(file.is_ok());
    let file = match file {
        Ok(url) => url,
        Err(error) => panic!("unexpected file URL parse failure: {error}"),
    };
    let replaced = file.mutator().host(Some("")).try_mutate();
    assert!(replaced.is_ok());
    let replaced = match replaced {
        Ok(url) => url,
        Err(error) => panic!("empty file host mutation failed: {error}"),
    };
    assert_eq!(replaced.host(), Some(""));
    assert_eq!(replaced.value(), "file:///a");
}

#[test]
/// Audits the public source declarations for unapproved URL surface additions.
fn public_abi_source_audit_success() {
    let source = include_str!("mod.rs");
    let declarations = source
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("///"))
        .collect::<Vec<_>>()
        .join("\n");
    let public_lines: Vec<&str> = source
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("pub struct ") || line.starts_with("pub fn "))
        .collect();
    assert_eq!(
        public_lines,
        [
            "pub struct URL {",
            "pub struct URLMutator {",
            "pub fn try_new(value: &str) -> Result<Self, Error> {",
            "pub fn value(&self) -> &str {",
            "pub fn scheme(&self) -> &str {",
            "pub fn username(&self) -> &str {",
            "pub fn password(&self) -> Option<&str> {",
            "pub fn host(&self) -> Option<&str> {",
            "pub fn port(&self) -> Option<u16> {",
            "pub fn path(&self) -> &str {",
            "pub fn query(&self) -> Option<&str> {",
            "pub fn query_pairs(&self) -> &[(String, String)] {",
            "pub fn fragment(&self) -> Option<&str> {",
            "pub fn has_opaque_path(&self) -> bool {",
            "pub fn try_resolve(&self, reference: &str) -> Result<Self, Error> {",
            "pub fn mutator(&self) -> URLMutator {",
            "pub fn scheme(self, value: &str) -> Self {",
            "pub fn username(self, value: &str) -> Self {",
            "pub fn password(self, value: Option<&str>) -> Self {",
            "pub fn host(self, value: Option<&str>) -> Self {",
            "pub fn port(self, value: Option<u16>) -> Self {",
            "pub fn path(self, value: &str) -> Self {",
            "pub fn query(self, value: Option<&str>) -> Self {",
            "pub fn fragment(self, value: Option<&str>) -> Self {",
            "pub fn try_mutate(self) -> Result<URL, Error> {",
        ]
    );
    for prohibited in ["pub mod engine", "pub use engine", "serde", "url::"] {
        assert!(!declarations.contains(prohibited));
    }
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
