//! Public URL conformance and compatibility integration tests.
//!
//! These suites exercise the crate exclusively through its supported public
//! API while retaining the pinned corpus counts and stable test names.

use kernel_oss::{
    error::{Audience, Error, Kind},
    values::{specification::repository_link::RepositoryLink, uri::url::URL},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[path = "generated/p6_vectors.rs"]
mod p6_vectors;
#[path = "generated/urltestdata_vectors.rs"]
mod urltestdata_vectors;

fn hash_of(value: &URL) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn parse_or_resolve(input: &str, base: Option<&str>) -> Result<URL, Error> {
    match base {
        Some(base) => URL::try_new(base)?.try_resolve(input),
        None => URL::try_new(input),
    }
}

#[test]
/// Verifies pinned WPT canonical serializations remain stable on a second parse.
fn pinned_urltestdata_parse_serialize_idempotence_success() {
    let mut successes = 0usize;
    for (index, vector) in urltestdata_vectors::VECTORS
        .iter()
        .enumerate()
        .filter(|(_, vector)| !vector.failure)
    {
        let first = match parse_or_resolve(vector.input, vector.base) {
            Ok(url) => url,
            Err(error) => panic!("unexpected failure at row {index}: {error}"),
        };
        let second = match URL::try_new(first.value()) {
            Ok(url) => url,
            Err(error) => panic!("canonical parse failed at row {index}: {error}"),
        };
        assert_eq!(second.value(), first.value(), "row {index}");
        assert_eq!(second, first, "record row {index}");
        successes += 1;
    }
    assert_eq!(successes, 624);
}

#[test]
/// Verifies the RFC 3986 normal and abnormal resolution examples under WHATWG parsing.
fn rfc_3986_resolution_vectors_success() {
    let base = match URL::try_new("http://a/b/c/d;p?q") {
        Ok(url) => url,
        Err(error) => panic!("base parse failed: {error}"),
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
        let resolved = match base.try_resolve(reference) {
            Ok(url) => url,
            Err(error) => panic!("failed reference {reference}: {error}"),
        };
        assert_eq!(resolved.value(), expected, "{reference}");
    }
}

#[test]
/// Verifies all pinned WPT URL projections through the public facade.
fn pinned_urltestdata_vectors_success() {
    let mut successes = 0usize;
    let mut failures = 0usize;
    for (index, vector) in urltestdata_vectors::VECTORS.iter().enumerate() {
        let result = parse_or_resolve(vector.input, vector.base);
        assert_eq!(
            result.is_err(),
            vector.failure,
            "unexpected outcome at row {index}"
        );
        if let Ok(url) = result {
            successes += 1;
            let hostname = url.host().unwrap_or_default();
            let port = url
                .port()
                .map_or_else(String::new, |value| value.to_string());
            let host = if port.is_empty() {
                hostname.to_owned()
            } else {
                format!("{hostname}:{port}")
            };
            let search = url.query().map_or_else(String::new, |value| {
                if value.is_empty() {
                    String::new()
                } else {
                    format!("?{value}")
                }
            });
            let hash = url.fragment().map_or_else(String::new, |value| {
                if value.is_empty() {
                    String::new()
                } else {
                    format!("#{value}")
                }
            });
            let pathname = if url.host().is_none() && !url.has_opaque_path() {
                url.path().strip_prefix("/.").unwrap_or(url.path())
            } else {
                url.path()
            };
            assert_eq!(Some(url.value()), vector.href, "href row {index}");
            assert_eq!(
                Some(format!("{}:", url.scheme()).as_str()),
                vector.protocol,
                "protocol row {index}"
            );
            assert_eq!(
                Some(url.username()),
                vector.username,
                "username row {index}"
            );
            assert_eq!(
                Some(url.password().unwrap_or_default()),
                vector.password,
                "password row {index}"
            );
            assert_eq!(Some(host.as_str()), vector.host, "host row {index}");
            assert_eq!(Some(hostname), vector.hostname, "hostname row {index}");
            assert_eq!(Some(port.as_str()), vector.port, "port row {index}");
            assert_eq!(Some(pathname), vector.pathname, "pathname row {index}");
            assert_eq!(Some(search.as_str()), vector.search, "search row {index}");
            assert_eq!(Some(hash.as_str()), vector.hash, "hash row {index}");
        } else {
            failures += 1;
        }
    }
    assert_eq!(urltestdata_vectors::VECTORS.len(), 891);
    assert_eq!(successes, 624);
    assert_eq!(failures, 267);
}

#[test]
/// Audits the public source declarations for unapproved URL surface additions.
fn public_abi_source_audit_success() {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/values/uri/url/mod.rs"
    ));
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
/// Qualifies every frozen P1A oracle case through the public dependency-free facade.
fn frozen_oracle_projection_success() {
    use p6_vectors::ORACLE_VECTORS;

    assert_eq!(ORACLE_VECTORS.len(), p6_vectors::ORACLE_CASE_COUNT);
    let mut comparison_references: Vec<(&str, URL)> = Vec::new();
    let mut parsed_cases = 0usize;
    let mut repository_cases = 0usize;

    for vector in ORACLE_VECTORS {
        let parsed = URL::try_new(vector.input);
        assert_eq!(
            parsed.is_ok(),
            vector.expected_success,
            "{} {} public parse outcome",
            vector.group,
            vector.id
        );
        if let Ok(url) = parsed {
            parsed_cases += 1;
            let Some(expected_value) = vector.expected_value else {
                panic!("{} lacks successful oracle value", vector.id);
            };
            let Some(expected_scheme) = vector.expected_scheme else {
                panic!("{} lacks successful oracle scheme", vector.id);
            };
            assert_eq!(url.value(), expected_value, "{} value", vector.id);
            assert_eq!(url.scheme(), expected_scheme, "{} scheme", vector.id);
            assert_eq!(
                url.username(),
                vector.expected_username.unwrap_or(""),
                "{} username",
                vector.id
            );
            assert_eq!(
                url.password().is_some(),
                vector.expected_password_present.unwrap_or(false),
                "{} password",
                vector.id
            );
            assert_eq!(url.host(), vector.expected_host, "{} host", vector.id);
            assert_eq!(url.port(), vector.expected_port, "{} port", vector.id);
            assert_eq!(
                url.path(),
                vector.expected_path.unwrap_or(""),
                "{} path",
                vector.id
            );
            assert_eq!(url.query(), vector.expected_query, "{} query", vector.id);
            let expected_pairs: Vec<(String, String)> = vector
                .expected_query_pairs
                .iter()
                .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
                .collect();
            assert_eq!(
                url.query_pairs(),
                expected_pairs.as_slice(),
                "{} query pairs",
                vector.id
            );
            assert_eq!(
                url.fragment(),
                vector.expected_fragment,
                "{} fragment",
                vector.id
            );
            if vector.group == "SEC-001" {
                assert_eq!(format!("{url:?}"), "URL([REDACTED])");
                for canary in ["canaryuser", "canarypass", "canaryquery", "canaryfragment"] {
                    assert!(
                        !format!("{url:?}").contains(canary),
                        "{} debug canary",
                        vector.id
                    );
                }
            }
            if let Some(class) = vector.comparison_class {
                if let Some((_, reference)) = comparison_references
                    .iter()
                    .find(|(existing, _)| *existing == class)
                {
                    assert_eq!(url, *reference, "{} equality class", vector.id);
                    assert_eq!(
                        hash_of(&url),
                        hash_of(reference),
                        "{} hash class",
                        vector.id
                    );
                } else {
                    comparison_references.push((class, url.clone()));
                }
            }
        } else {
            let error = match parsed {
                Ok(_) => panic!("oracle case unexpectedly parsed"),
                Err(error) => error,
            };
            assert_eq!(error.kind(), Kind::InvalidInput, "{} error kind", vector.id);
            assert_eq!(
                error.audience(),
                Audience::User,
                "{} error audience",
                vector.id
            );
            assert_eq!(
                error.message(),
                "URL parsing failed.",
                "{} error text",
                vector.id
            );
        }

        if vector.operation == "repository_link" {
            repository_cases += 1;
            let mut builder = RepositoryLink::builder();
            if vector.allowed_schemes_supplied {
                builder = builder.allowed_schema(
                    vector
                        .allowed_schemes
                        .iter()
                        .map(|scheme| (*scheme).to_owned())
                        .collect(),
                );
            }
            if vector.default_scheme_supplied {
                let Some(default_scheme) = vector.default_scheme else {
                    panic!("{} lacks supplied default scheme", vector.id);
                };
                builder = builder.default_scheme(default_scheme);
            }
            if vector.repo_link_supplied {
                builder = builder.repo_link(vector.input);
            }
            let result = builder.build();
            assert_eq!(
                result.is_ok(),
                vector.repository_expected_success.unwrap_or(false),
                "{} repository outcome",
                vector.id
            );
            match result {
                Ok(link) => {
                    let Some(expected_value) = vector.repository_url_value else {
                        panic!("{} lacks successful repository URL value", vector.id);
                    };
                    let Some(expected_scheme) = vector.repository_url_scheme else {
                        panic!("{} lacks successful repository URL scheme", vector.id);
                    };
                    assert_eq!(
                        link.url().value(),
                        expected_value,
                        "{} repository URL value",
                        vector.id
                    );
                    assert_eq!(
                        link.url().scheme(),
                        expected_scheme,
                        "{} repository URL scheme",
                        vector.id
                    );
                    assert_eq!(
                        link.url().username(),
                        vector.repository_url_username.unwrap_or(""),
                        "{} repository URL username",
                        vector.id
                    );
                    assert_eq!(
                        link.url().password().is_some(),
                        vector.repository_url_password_present.unwrap_or(false),
                        "{} repository URL password",
                        vector.id
                    );
                    assert_eq!(
                        link.url().host(),
                        vector.repository_url_host,
                        "{} repository URL host",
                        vector.id
                    );
                    assert_eq!(
                        link.url().port(),
                        vector.repository_url_port,
                        "{} repository URL port",
                        vector.id
                    );
                    assert_eq!(
                        link.url().path(),
                        vector.repository_url_path.unwrap_or(""),
                        "{} repository URL path",
                        vector.id
                    );
                    assert_eq!(
                        link.url().query(),
                        vector.repository_url_query,
                        "{} repository URL query",
                        vector.id
                    );
                    let expected_pairs: Vec<(String, String)> = vector
                        .repository_url_query_pairs
                        .iter()
                        .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
                        .collect();
                    assert_eq!(
                        link.url().query_pairs(),
                        expected_pairs.as_slice(),
                        "{} repository URL query pairs",
                        vector.id
                    );
                    assert_eq!(
                        link.url().fragment(),
                        vector.repository_url_fragment,
                        "{} repository URL fragment",
                        vector.id
                    );
                    let default_scheme_applied = !vector.input.contains("://");
                    assert_eq!(
                        Some(default_scheme_applied),
                        vector.repository_default_scheme_applied,
                        "{} repository default-scheme classification",
                        vector.id
                    );
                    if default_scheme_applied {
                        assert_eq!(
                            Some(link.url().scheme()),
                            vector.default_scheme,
                            "{} repository default scheme",
                            vector.id
                        );
                    }
                    assert_eq!(format!("{link}"), "RepositoryLink([REDACTED])");
                    assert_eq!(format!("{link:?}"), "RepositoryLink([REDACTED])");
                }
                Err(error) => {
                    assert_eq!(
                        vector.repository_expected_kind,
                        Some("INVALID_INPUT"),
                        "{} repository oracle category",
                        vector.id
                    );
                    assert_eq!(
                        error.kind(),
                        Kind::InvalidInput,
                        "{} repository kind",
                        vector.id
                    );
                    let expected_audience = if vector.repository_expected_audience == Some("SYSTEM")
                    {
                        Audience::System
                    } else {
                        Audience::User
                    };
                    assert_eq!(
                        error.audience(),
                        expected_audience,
                        "{} repository audience",
                        vector.id
                    );
                    assert_eq!(error.message(), "Repository link validation failed.");
                }
            }
        }
    }
    assert_eq!(parsed_cases + 46, p6_vectors::ORACLE_CASE_COUNT);
    assert_eq!(repository_cases, 21);
    assert_eq!(comparison_references.len(), 3);
}

#[test]
/// Maps every representable WPT URL setter case onto immutable URLMutator semantics.
fn pinned_wpt_setter_vectors_success() {
    use p6_vectors::{SETTER_VECTORS, SetterAttribute, SetterClassification};

    assert_eq!(SETTER_VECTORS.len(), p6_vectors::SETTER_SOURCE_CASE_COUNT);
    let mut mapped = 0usize;
    let mut fixed_divergence = 0usize;
    let mut browser_only = 0usize;

    for vector in SETTER_VECTORS {
        if matches!(
            vector.classification,
            SetterClassification::BrowserOnlyHref | SetterClassification::BrowserOnlyOpaquePath
        ) {
            browser_only += 1;
            assert!(matches!(
                vector.attribute,
                SetterAttribute::BrowserOnlyHref | SetterAttribute::Path
            ));
            continue;
        }
        let base = match URL::try_new(vector.href) {
            Ok(url) => url,
            Err(error) => panic!("setter base failed: {error}"),
        };
        let result = match vector.attribute {
            SetterAttribute::Scheme => base
                .mutator()
                .scheme(
                    vector
                        .new_value
                        .chars()
                        .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
                        .collect::<String>()
                        .split(':')
                        .next()
                        .unwrap_or(vector.new_value),
                )
                .try_mutate(),
            SetterAttribute::Username => base.mutator().username(vector.new_value).try_mutate(),
            SetterAttribute::Password => {
                base.mutator().password(Some(vector.new_value)).try_mutate()
            }
            SetterAttribute::Host | SetterAttribute::Hostname => {
                base.mutator().host(Some(vector.new_value)).try_mutate()
            }
            SetterAttribute::Port => {
                let digits: String = vector
                    .new_value
                    .chars()
                    .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
                    .take_while(char::is_ascii_digit)
                    .collect();
                if vector.new_value.is_empty() {
                    base.mutator().port(None).try_mutate()
                } else if let Ok(port) = digits.parse::<u16>() {
                    base.mutator().port(Some(port)).try_mutate()
                } else {
                    Err(Error::for_user(Kind::InvalidInput, "URL mutation failed."))
                }
            }
            SetterAttribute::Path => base.mutator().path(vector.new_value).try_mutate(),
            SetterAttribute::Query => {
                let value = vector
                    .new_value
                    .strip_prefix('?')
                    .unwrap_or(vector.new_value);
                let replacement = (!vector.new_value.is_empty()).then_some(value);
                base.mutator().query(replacement).try_mutate()
            }
            SetterAttribute::Fragment => {
                let value = vector
                    .new_value
                    .strip_prefix('#')
                    .unwrap_or(vector.new_value);
                let replacement = (!vector.new_value.is_empty()).then_some(value);
                base.mutator().fragment(replacement).try_mutate()
            }
            SetterAttribute::BrowserOnlyHref => panic!("browser-only href reached mapping"),
        };
        match vector.classification {
            SetterClassification::Mapped => {
                let url = match result {
                    Ok(url) => url,
                    Err(error) => panic!(
                        "mapped setter unexpectedly failed for {} -> {}: {error}",
                        vector.href, vector.new_value
                    ),
                };
                mapped += 1;
                assert_eq!(
                    url.value(),
                    vector.expected_href,
                    "mapped setter output for {} -> {}",
                    vector.href,
                    vector.new_value
                );
            }
            SetterClassification::BrowserOnlyHref | SetterClassification::BrowserOnlyOpaquePath => {
                panic!("browser-only classification reached mapping")
            }
            SetterClassification::FixedHostNoAuthority => {
                fixed_divergence += 1;
                // Browser setters leave opaque bases unchanged.  This frozen
                // immutable API permits an explicit host replacement to select
                // an authority-bearing record; that semantic difference is
                // classified from fixture input rather than expected output.
                if let Ok(url) = result {
                    assert_eq!(url.port(), base.port(), "fixed no-authority host port");
                    assert_eq!(url.query(), base.query(), "fixed no-authority host query");
                    assert_eq!(
                        url.fragment(),
                        base.fragment(),
                        "fixed no-authority host fragment"
                    );
                }
            }
            SetterClassification::FixedHostBrowserNoOp => {
                fixed_divergence += 1;
                // Browser setters silently ignore these inputs.  The immutable
                // component API instead either rejects them or applies its
                // host-component canonicalization; it never manufactures an
                // authority, port, path, query, or fragment from the input.
                if let Ok(url) = result {
                    assert_eq!(url.port(), base.port(), "fixed host divergence port");
                    assert_eq!(url.path(), base.path(), "fixed host divergence path");
                    assert_eq!(url.query(), base.query(), "fixed host divergence query");
                    assert_eq!(
                        url.fragment(),
                        base.fragment(),
                        "fixed host divergence fragment"
                    );
                }
            }
            SetterClassification::FixedHostPreprocessing
            | SetterClassification::FixedHostDelimiter
            | SetterClassification::FixedHostCombinedPortOrInvalid
            | SetterClassification::FixedHostEmptySpecial
            | SetterClassification::FixedHostEmptyWithCredentials
            | SetterClassification::FixedHostEmptyWithPort
            | SetterClassification::FixedHostMappedEmpty => {
                fixed_divergence += 1;
                let error = match result {
                    Ok(_) => panic!(
                        "security-boundary host divergence unexpectedly succeeded for {} -> {}",
                        vector.href, vector.new_value
                    ),
                    Err(error) => error,
                };
                assert_eq!(error.kind(), Kind::InvalidInput);
                assert_eq!(error.audience(), Audience::User);
                assert_eq!(error.message(), "URL mutation failed.");
            }
            SetterClassification::FixedSchemeInvalid => {
                fixed_divergence += 1;
                let error = match result {
                    Ok(_) => panic!("invalid browser scheme setter unexpectedly succeeded"),
                    Err(error) => error,
                };
                assert_eq!(error.kind(), Kind::InvalidInput);
                assert_eq!(error.audience(), Audience::User);
                assert_eq!(error.message(), "URL mutation failed.");
            }
            SetterClassification::FixedSchemeStructural
            | SetterClassification::FixedFileCredentials
            | SetterClassification::FixedFilePort
            | SetterClassification::FixedNoAuthorityPort
            | SetterClassification::FixedEmptyHostPort
            | SetterClassification::FixedNoAuthorityCredentials
            | SetterClassification::FixedEmptyHostCredentials
            | SetterClassification::FixedPortInvalid => {
                fixed_divergence += 1;
                let error = match result {
                    Ok(_) => panic!("fixed structural setter unexpectedly succeeded"),
                    Err(error) => error,
                };
                assert_eq!(error.kind(), Kind::InvalidInput);
                assert_eq!(error.audience(), Audience::User);
                assert_eq!(error.message(), "URL mutation failed.");
            }
        }
    }
    assert_eq!(mapped, p6_vectors::SETTER_MAPPED_CASE_COUNT);
    assert_eq!(
        fixed_divergence,
        p6_vectors::SETTER_FIXED_DIVERGENCE_CASE_COUNT
    );
    assert_eq!(browser_only, p6_vectors::SETTER_BROWSER_ONLY_CASE_COUNT);
    assert_eq!(
        mapped + fixed_divergence + browser_only,
        SETTER_VECTORS.len()
    );
}

#[test]
/// Keeps the public URL facade independent of the former external URL crate.
fn dependency_free_url_cutover_success() {
    let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    let lockfile = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.lock"));
    assert!(
        !manifest
            .lines()
            .any(|line| line.trim_start().starts_with("url ="))
    );
    assert!(!lockfile.contains("name = \"url\""));
    assert!(!lockfile.contains("name = \"idna\""));
    assert!(!lockfile.contains("name = \"form_urlencoded\""));
    assert!(!lockfile.contains("name = \"percent-encoding\""));
}
