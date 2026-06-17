//! Verifies the bounded URL value object.
//!
//! Bounded unit under test:
//! - `URL`
//!
//! Public interfaces verified:
//! - `URL::new`
//! - `URL::value`
//! - `URL::line`
//! - `URL::scheme`
//! - `URL::host`
//! - `URL::port`
//! - `URL::path`
//! - `URL::query_string`
//! - `URL::queries`
//! - `URL::query_pairs`
//! - `URL::query_count`
//! - `URL::fragment`
//!
//! Logical paths covered:
//! - valid URL parsing succeeds with scheme, host, port, path, query, and fragment
//! - URLs with only scheme and host default the remaining fields
//! - multiple query parameters are preserved
//! - malformed URL input is rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::URL;
use crate::error::{Audience, Kind};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a fully populated URL parses successfully and preserves all
/// bounded parts.
#[test]
fn create_new_url_success() {
    let url_result = URL::new("http://www.example.com:8080/some/path?query=value#the-fragment");
    match url_result {
        Ok(url) => {
            assert_eq!(
                url.value(),
                "http://www.example.com:8080/some/path?query=value#the-fragment"
            );
            assert_eq!(url.scheme(), "http");
            assert_eq!(url.host(), "www.example.com");
            assert_eq!(url.port(), 8080);
            assert_eq!(url.path(), "/some/path");
            assert_eq!(url.query_string(), "query=value");
            assert_eq!(url.query_count(), 1);
            assert_eq!(url.fragment(), "the-fragment");
            assert_eq!(
                url.line().value(),
                "http://www.example.com:8080/some/path?query=value#the-fragment"
            );
            assert_eq!(
                url.query_pairs(),
                &[("query".to_string(), "value".to_string())]
            );
        }
        Err(error) => {
            panic!(
                "An error was returned. Did not expect an error: [{0}]",
                error.message
            )
        }
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that URLs with only scheme and host normalize the missing parts.
#[test]
fn create_new_url_only_scheme_and_host_success() {
    let url_result = URL::new("https://www.example.com");
    match url_result {
        Ok(url) => {
            assert_eq!(url.value(), "https://www.example.com");
            assert_eq!(url.scheme(), "https");
            assert_eq!(url.host(), "www.example.com");
            assert_eq!(url.port(), 0);
            assert_eq!(url.path(), "/");
            assert_eq!(url.query_string(), "");
            assert_eq!(url.query_count(), 0);
            assert_eq!(url.fragment(), "");
            assert_eq!(url.line().value(), "https://www.example.com");
            assert!(url.query_pairs().is_empty());
        }
        Err(error) => {
            panic!(
                "An error was returned. Did not expect an error: [{0}]",
                error.message
            )
        }
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that multiple query parameters survive parsing and storage.
#[test]
fn create_new_url_multiple_queries_success() {
    let url_result =
        URL::new("http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment");
    match url_result {
        Ok(url) => {
            assert_eq!(
                url.value(),
                "http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment"
            );
            assert_eq!(url.scheme(), "http");
            assert_eq!(url.host(), "www.example.com");
            assert_eq!(url.port(), 8080);
            assert_eq!(url.path(), "/some/path");
            assert_eq!(url.query_string(), "query1=value1&query2=value2");
            assert_eq!(url.query_count(), 2);
            assert!(
                url.query_pairs()
                    .contains(&("query1".to_string(), "value1".to_string()))
            );
            assert!(
                url.query_pairs()
                    .contains(&("query2".to_string(), "value2".to_string()))
            );
            assert_eq!(url.fragment(), "fragment");
            assert_eq!(
                url.line().value(),
                "http://www.example.com:8080/some/path?query1=value1&query2=value2#fragment"
            );
        }
        Err(error) => {
            panic!(
                "An error was returned. Did not expect an error: [{0}]",
                error.message
            )
        }
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that the accessor methods return the parsed URL components.
#[test]
fn accessors_success() {
    let url = URL::new("https://example.com/path?one=1&two=2#frag").expect("expected valid url");

    assert_eq!(
        url.line().value(),
        "https://example.com/path?one=1&two=2#frag"
    );
    assert_eq!(url.scheme(), "https");
    assert_eq!(url.host(), "example.com");
    assert_eq!(url.port(), 0);
    assert_eq!(url.path(), "/path");
    assert_eq!(url.query_string(), "one=1&two=2");
    assert_eq!(
        url.query_pairs(),
        &[
            ("one".to_string(), "1".to_string()),
            ("two".to_string(), "2".to_string())
        ]
    );
    assert_eq!(url.query_count(), 2);
    assert_eq!(url.fragment(), "frag");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that malformed URL input is rejected with a bounded user error.
#[test]
fn create_from_bad_url_input_error() {
    let url_result = URL::new("Some-Bad-Input");
    match url_result {
        Ok(_url) => {
            panic!("Was expecting an error, but the URL::new(...) processed successfully.");
        }
        Err(error) => {
            assert_eq!(error.kind, Kind::InvalidInput);
            assert_eq!(error.audience, Audience::User);
            assert!(error.message.starts_with("You provided an invalid Url. "));
            assert!(error.message.contains("Your Input: "));
            assert!(error.message.contains("The Issue: "));
        }
    }
}
