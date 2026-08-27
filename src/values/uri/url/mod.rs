//! Public, redaction-safe URL facade backed by the private WHATWG engine.
//!
//! [`URL::value`] returns the complete canonical serialization, which can contain
//! credentials, query data, and a fragment. Callers must treat it as sensitive.

use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::{
    error::{Error, Kind},
    values::Value,
};

mod engine;

use engine::{
    form_urlencoded, host, parser,
    percent_encoding::{self, EncodeSet},
    record::{Path, Record},
    serializer,
};

/// A finalized, canonical URL with private representation state.
///
/// # Intentionally absent legacy surfaces
///
/// Construction is fallible and there is no `URL::new` compatibility shim.
/// ```compile_fail
/// use kernel_oss::values::uri::url::URL;
/// let _ = URL::new("https://example.test/");
/// ```
///
/// State is private, and the former `line` projection is not part of this API.
/// ```compile_fail
/// use kernel_oss::values::uri::url::URL;
/// let url = URL::try_new("https://example.test/").unwrap();
/// let _ = url.host;
/// let _ = url.line();
/// ```
///
/// Automatic string formatting is deliberately unavailable for sensitive URLs.
/// ```compile_fail
/// use kernel_oss::values::uri::url::URL;
/// let url = URL::try_new("https://example.test/").unwrap();
/// let _ = format!("{url}");
/// ```
///
/// URL does not implement copy, default, ordering, or parsing conversion traits.
/// ```compile_fail
/// use kernel_oss::values::uri::url::URL;
/// use std::str::FromStr;
/// fn needs_copy<T: Copy>() {}
/// fn needs_default<T: Default>() {}
/// fn needs_order<T: Ord>() {}
/// needs_copy::<URL>();
/// needs_default::<URL>();
/// needs_order::<URL>();
/// let _ = URL::from_str("https://example.test/");
/// ```
///
/// The mutator's fields and private engine representations are not public.
/// ```compile_fail
/// use kernel_oss::values::uri::url::URL;
/// let mutator = URL::try_new("https://example.test/").unwrap().mutator();
/// let _ = mutator.host;
/// let _ = kernel_oss::values::uri::url::engine::parser::parse;
/// ```
#[derive(Clone)]
pub struct URL {
    record: Record,
    value: String,
    host: Option<String>,
    path: String,
    query_pairs: Vec<(String, String)>,
}

/// A deferred, immutable reconstruction of a [`URL`].
pub struct URLMutator {
    url: URL,
    scheme: Option<String>,
    username: Option<String>,
    password: Option<Option<String>>,
    host: Option<Option<String>>,
    port: Option<Option<u16>>,
    path: Option<String>,
    query: Option<Option<String>>,
    fragment: Option<Option<String>>,
}

impl URL {
    /// Parses `value` through the private canonical WHATWG engine.
    pub fn try_new(value: &str) -> Result<Self, Error> {
        parser::parse(value, None)
            .map(|outcome| Self::from_record(outcome.record))
            .map_err(|_| parsing_error())
    }

    /// Returns the complete canonical serialization.
    ///
    /// This value can contain credentials, query data, and a fragment. Treat it as
    /// sensitive and do not use it for automatic diagnostics or formatting.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the canonical URL scheme.
    pub fn scheme(&self) -> &str {
        self.record.scheme()
    }

    /// Returns the canonical username, or an empty value when authority is absent.
    pub fn username(&self) -> &str {
        self.record.username()
    }

    /// Returns the canonical password when it is present.
    pub fn password(&self) -> Option<&str> {
        self.record.password()
    }

    /// Returns the canonical serialized host when authority is present.
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    /// Returns the explicit, non-default port when it is present.
    pub fn port(&self) -> Option<u16> {
        self.record.port()
    }

    /// Returns the canonical path serialization.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the canonical encoded query when the query delimiter is present.
    pub fn query(&self) -> Option<&str> {
        self.record.query()
    }

    /// Returns decoded form query pairs in source order without allocating.
    pub fn query_pairs(&self) -> &[(String, String)] {
        &self.query_pairs
    }

    /// Returns the canonical fragment when the fragment delimiter is present.
    pub fn fragment(&self) -> Option<&str> {
        self.record.fragment()
    }

    /// Returns whether this URL uses a cannot-be-a-base opaque path.
    pub fn has_opaque_path(&self) -> bool {
        self.record.has_opaque_path()
    }

    /// Resolves an absolute or relative reference using this URL as its immutable base.
    pub fn try_resolve(&self, reference: &str) -> Result<Self, Error> {
        parser::parse(reference, Some(&self.record))
            .map(|outcome| Self::from_record(outcome.record))
            .map_err(|_| resolution_error())
    }

    /// Starts a deferred immutable reconstruction of this URL.
    pub fn mutator(&self) -> URLMutator {
        URLMutator {
            url: self.clone(),
            scheme: None,
            username: None,
            password: None,
            host: None,
            port: None,
            path: None,
            query: None,
            fragment: None,
        }
    }

    fn from_record(record: Record) -> Self {
        let value = serializer::serialize(&record);
        let host = record.host().map(host::serialize);
        let path = match record.path() {
            Path::Segments(segments) => segments.join("/"),
            Path::Opaque(value) => value.clone(),
        };
        let query_pairs = record.query().map_or_else(Vec::new, form_urlencoded::parse);
        Self {
            record,
            value,
            host,
            path,
            query_pairs,
        }
    }
}

impl Value for URL {
    type ValueType = str;

    /// Returns the sensitive complete canonical serialization.
    fn value(&self) -> &Self::ValueType {
        self.value()
    }
}

impl PartialEq for URL {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for URL {}

impl Hash for URL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl fmt::Debug for URL {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("URL([REDACTED])")
    }
}

impl URLMutator {
    /// Requests a replacement scheme.
    pub fn scheme(mut self, value: &str) -> Self {
        self.scheme = Some(value.to_owned());
        self
    }

    /// Requests a replacement username.
    pub fn username(mut self, value: &str) -> Self {
        self.username = Some(value.to_owned());
        self
    }

    /// Requests a replacement or removal of the password.
    pub fn password(mut self, value: Option<&str>) -> Self {
        self.password = Some(value.map(str::to_owned));
        self
    }

    /// Requests a replacement or removal of the host.
    pub fn host(mut self, value: Option<&str>) -> Self {
        self.host = Some(value.map(str::to_owned));
        self
    }

    /// Requests a replacement or removal of the explicit port.
    pub fn port(mut self, value: Option<u16>) -> Self {
        self.port = Some(value);
        self
    }

    /// Requests a replacement path component.
    pub fn path(mut self, value: &str) -> Self {
        self.path = Some(value.to_owned());
        self
    }

    /// Requests a replacement or removal of the query component.
    pub fn query(mut self, value: Option<&str>) -> Self {
        self.query = Some(value.map(str::to_owned));
        self
    }

    /// Requests a replacement or removal of the fragment component.
    pub fn fragment(mut self, value: Option<&str>) -> Self {
        self.fragment = Some(value.map(str::to_owned));
        self
    }

    /// Reconstructs and validates one new canonical URL from all replacements.
    pub fn try_mutate(self) -> Result<URL, Error> {
        let scheme = self
            .scheme
            .unwrap_or_else(|| self.url.scheme().to_owned())
            .to_ascii_lowercase();
        if !is_bare_scheme(&scheme) {
            return Err(mutation_error());
        }

        let username = self
            .username
            .unwrap_or_else(|| self.url.username().to_owned());
        let password = self
            .password
            .unwrap_or_else(|| self.url.password().map(str::to_owned));
        let host_replaced = self.host.is_some();
        let host = self
            .host
            .unwrap_or_else(|| self.url.host().map(str::to_owned));
        let port = self.port.unwrap_or_else(|| self.url.port());
        let path = self.path.unwrap_or_else(|| self.url.path().to_owned());
        let query = self
            .query
            .unwrap_or_else(|| self.url.query().map(str::to_owned));
        let fragment = self
            .fragment
            .unwrap_or_else(|| self.url.fragment().map(str::to_owned));
        let has_authority = if host_replaced {
            host.is_some()
        } else {
            self.url.record.has_authority()
        };

        if host
            .as_deref()
            .is_some_and(|value| !is_host_component(value))
            || !has_authority && (!username.is_empty() || password.is_some() || port.is_some())
            || is_special_scheme(&scheme) && !has_authority
            || !has_authority && path.starts_with("//")
        {
            return Err(mutation_error());
        }

        let mut value = format!("{scheme}:");
        if has_authority {
            let Some(host) = host else {
                return Err(mutation_error());
            };
            value.push_str("//");
            if !username.is_empty() || password.is_some() {
                value.push_str(&percent_encoding::encode(&username, EncodeSet::UserInfo));
                if let Some(password) = password {
                    value.push(':');
                    value.push_str(&percent_encoding::encode(&password, EncodeSet::UserInfo));
                }
                value.push('@');
            }
            value.push_str(&host);
            if let Some(port) = port {
                value.push(':');
                value.push_str(&port.to_string());
            }
        }
        if has_authority && !path.is_empty() && !path.starts_with('/') {
            value.push('/');
        }
        value.push_str(&percent_encoding::encode(&path, EncodeSet::Path));
        if let Some(query) = query {
            value.push('?');
            let set = if is_special_scheme(&scheme) {
                EncodeSet::SpecialQuery
            } else {
                EncodeSet::Query
            };
            value.push_str(&percent_encoding::encode(&query, set));
        }
        if let Some(fragment) = fragment {
            value.push('#');
            value.push_str(&percent_encoding::encode(&fragment, EncodeSet::Fragment));
        }
        URL::try_new(&value).map_err(|_| mutation_error())
    }
}

fn is_bare_scheme(value: &str) -> bool {
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z'))
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
}

fn is_host_component(value: &str) -> bool {
    !value
        .chars()
        .any(|character| matches!(character, '@' | '/' | '\\' | '?' | '#'))
}

fn is_special_scheme(value: &str) -> bool {
    matches!(value, "ftp" | "file" | "http" | "https" | "ws" | "wss")
}

fn parsing_error() -> Error {
    Error::for_user(Kind::InvalidInput, "URL parsing failed.")
}

fn resolution_error() -> Error {
    Error::for_user(Kind::InvalidInput, "URL resolution failed.")
}

fn mutation_error() -> Error {
    Error::for_user(Kind::InvalidInput, "URL mutation failed.")
}

#[cfg(test)]
mod tests;
