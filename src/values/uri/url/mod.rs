use crate::error::{Error, Kind};
use crate::values::text::line::Line;
use url::{Host, Url};

/// A parsed URL value with exposed components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct URL {
    /// The original URL text.
    pub value: Line,
    /// The URL scheme.
    pub scheme: String,
    /// The URL host.
    pub host: String,
    /// The URL port.
    pub port: u16,
    /// The URL path.
    pub path: String,
    /// The raw URL query string.
    pub query_string: String,
    /// The parsed query pairs.
    pub queries: Vec<(String, String)>,
    /// The URL fragment.
    pub fragment: String,
}

impl URL {
    /// Parses a URL string into a structured value.
    pub fn new(url: &str) -> Result<Self, Error> {
        let raw_input = Line::new(url);
        Url::parse(&raw_input.value())
            .map_err(|error| {
                Error::for_user(
                    Kind::InvalidInput,
                    format!(
                        "You provided an invalid Url. \
                            Your Input: [{0}], The Issue: [{1}]",
                        raw_input.value(),
                        error
                    ),
                )
            })
            .map(|parsed_url| URL {
                value: raw_input,
                scheme: parsed_url.scheme().to_string(),
                host: parsed_url.host().unwrap_or(Host::Domain("")).to_string(),
                port: parsed_url.port().unwrap_or(0),
                path: parsed_url.path().to_string(),
                query_string: parsed_url.query().unwrap_or("").to_string(),
                queries: extract_multiple_queries(&parsed_url),
                fragment: parsed_url.fragment().unwrap_or("").to_string(),
            })
    }
    /// Returns the original URL text.
    pub fn value(&self) -> String {
        self.value.value()
    }
    /// Returns the underlying line value.
    pub fn line(&self) -> &Line {
        &self.value
    }
    /// Returns the URL scheme.
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
    /// Returns the URL host.
    pub fn host(&self) -> &str {
        &self.host
    }
    /// Returns the URL port.
    pub fn port(&self) -> u16 {
        self.port
    }
    /// Returns the URL path.
    pub fn path(&self) -> &str {
        &self.path
    }
    /// Returns the raw query string.
    pub fn query_string(&self) -> &str {
        &self.query_string
    }
    /// Returns the parsed query pairs as owned values.
    pub fn queries(&self) -> Vec<(String, String)> {
        self.queries.clone()
    }
    /// Returns the parsed query pairs as a slice.
    pub fn query_pairs(&self) -> &[(String, String)] {
        &self.queries
    }
    /// Returns the number of query pairs.
    pub fn query_count(&self) -> usize {
        self.queries.len()
    }
    /// Returns the URL fragment.
    pub fn fragment(&self) -> &str {
        &self.fragment
    }
}

fn extract_multiple_queries(parsed_url: &Url) -> Vec<(String, String)> {
    let mut queries: Vec<(String, String)> = Vec::new();
    for (key, value) in parsed_url.query_pairs() {
        queries.push((key.into_owned(), value.into_owned()));
    }
    queries
}

#[cfg(test)]
mod tests;
