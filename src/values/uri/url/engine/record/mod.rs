//! Private canonical URL state shared by the P4 parser, resolver, and serializer.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "P4 private URL state is wired to the public facade in P5."
    )
)]

#[cfg(test)]
mod tests;

use super::{
    host::{self, Host},
    percent_encoding::{self, EncodeSet},
};

/// The mutually exclusive canonical path representations defined by the URL model.
#[derive(Clone, Eq, PartialEq)]
pub(super) enum Path {
    /// A slash-separated hierarchical path. Empty segments preserve delimiters.
    Segments(Vec<String>),
    /// An opaque path for cannot-be-a-base URLs.
    Opaque(String),
}

/// Canonical, private URL state. It deliberately implements no diagnostic formatting.
#[derive(Clone, Eq, PartialEq)]
pub(super) struct Record {
    scheme: String,
    authority: Option<Authority>,
    path: Path,
    query: Option<String>,
    fragment: Option<String>,
}

/// Canonical authority state. It can only exist together with a host.
#[derive(Clone, Eq, PartialEq)]
pub(super) struct Authority {
    username: String,
    password: Option<String>,
    host: Host,
    port: Option<u16>,
}

impl Record {
    /// Creates a canonical record while enforcing representation-level invariants.
    pub(super) fn new(
        scheme: String,
        mut authority: Option<Authority>,
        path: Path,
        query: Option<String>,
        fragment: Option<String>,
    ) -> Result<Self, Error> {
        if !is_canonical_scheme(&scheme)
            || matches!(path, Path::Opaque(_)) && authority.is_some()
            || is_special_scheme(&scheme) && matches!(path, Path::Opaque(_))
            || is_special_scheme(&scheme) && authority.is_none()
            || is_special_scheme(&scheme) && !is_rooted_path(&path)
            || is_special_scheme(&scheme) && is_single_empty_segment_path(&path)
            || authority.is_some() && !is_rooted_path(&path) && !is_empty_segments(&path)
            || !is_special_scheme(&scheme)
                && authority.is_none()
                && matches!(path, Path::Segments(_))
                && (!is_rooted_path(&path) || is_empty_segments(&path))
        {
            return Err(Error::Invalid);
        }
        if let Some(authority) = &mut authority {
            if is_special_scheme(&scheme) && matches!(authority.host, Host::Opaque(_)) {
                return Err(Error::Invalid);
            }
            if !is_special_scheme(&scheme)
                && matches!(authority.host, Host::Domain(_) | Host::Ipv4(_))
            {
                return Err(Error::Invalid);
            }
            if scheme != "file"
                && is_special_scheme(&scheme)
                && matches!(authority.host, Host::Empty)
            {
                return Err(Error::Invalid);
            }
            if scheme == "file"
                && (!authority.username.is_empty()
                    || authority.password.is_some()
                    || authority.port.is_some())
            {
                return Err(Error::Invalid);
            }
            if matches!(authority.host, Host::Empty)
                && (!authority.username.is_empty()
                    || authority.password.is_some()
                    || authority.port.is_some())
            {
                return Err(Error::Invalid);
            }
            if !is_canonical_authority(&scheme, authority) {
                return Err(Error::Invalid);
            }
            if authority.password.as_deref() == Some("") {
                authority.password = None;
            }
            if is_special_scheme(&scheme) && default_port(&scheme) == authority.port {
                authority.port = None;
            }
        }
        if !is_canonical_path(&path)
            || !is_canonical_opaque_path(&path)
            || !is_canonical_query(query.as_deref(), is_special_scheme(&scheme))
            || !is_canonical_fragment(fragment.as_deref())
        {
            return Err(Error::Invalid);
        }
        Ok(Self {
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }

    /// Returns the canonical scheme.
    pub(super) fn scheme(&self) -> &str {
        &self.scheme
    }

    /// Returns whether this scheme is special under the URL standard.
    pub(super) fn is_special(&self) -> bool {
        is_special_scheme(&self.scheme)
    }

    /// Returns whether this is a file URL.
    pub(super) fn is_file(&self) -> bool {
        self.scheme == "file"
    }

    /// Returns whether this record contains authority state.
    pub(super) fn has_authority(&self) -> bool {
        self.authority.is_some()
    }

    /// Returns the canonical username, or an empty value when authority is absent.
    pub(super) fn username(&self) -> &str {
        self.authority
            .as_ref()
            .map_or("", |authority| &authority.username)
    }

    /// Returns the canonical optional password.
    pub(super) fn password(&self) -> Option<&str> {
        self.authority
            .as_ref()
            .and_then(|authority| authority.password.as_deref())
    }

    /// Returns the authority host when present.
    pub(super) fn host(&self) -> Option<&Host> {
        self.authority.as_ref().map(|authority| &authority.host)
    }

    /// Returns the canonical non-default port when present.
    pub(super) fn port(&self) -> Option<u16> {
        self.authority.as_ref().and_then(|authority| authority.port)
    }

    /// Returns the canonical path.
    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the canonical optional query.
    pub(super) fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// Returns the canonical optional fragment.
    pub(super) fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }

    /// Returns a new record with a replacement path.
    pub(super) fn with_path(&self, path: Path) -> Result<Self, Error> {
        Self::new(
            self.scheme.clone(),
            self.authority.clone(),
            path,
            self.query.clone(),
            self.fragment.clone(),
        )
    }

    /// Returns a new record with a replacement query.
    pub(super) fn with_query(&self, query: Option<String>) -> Result<Self, Error> {
        Self::new(
            self.scheme.clone(),
            self.authority.clone(),
            self.path.clone(),
            query,
            self.fragment.clone(),
        )
    }

    /// Returns a new record with a replacement fragment.
    pub(super) fn with_fragment(&self, fragment: Option<String>) -> Result<Self, Error> {
        Self::new(
            self.scheme.clone(),
            self.authority.clone(),
            self.path.clone(),
            self.query.clone(),
            fragment,
        )
    }

    /// Returns whether this record cannot serve as a relative-resolution base.
    pub(super) fn has_opaque_path(&self) -> bool {
        matches!(self.path, Path::Opaque(_))
    }
}

fn is_canonical_authority(scheme: &str, authority: &Authority) -> bool {
    if percent_encoding::encode(&authority.username, EncodeSet::UserInfo) != authority.username
        || authority.password.as_ref().is_some_and(|password| {
            percent_encoding::encode(password, EncodeSet::UserInfo) != *password
        })
    {
        return false;
    }
    let canonical_host = match &authority.host {
        Host::Domain(value) => {
            !value.is_empty()
                && !(scheme == "file" && value == "localhost")
                && host::parse_special(value).is_ok_and(|outcome| outcome.value == authority.host)
        }
        Host::Opaque(value) => {
            !value.is_empty()
                && host::parse_opaque(value).is_ok_and(|outcome| outcome.value == authority.host)
        }
        Host::Ipv4(_) | Host::Ipv6(_) | Host::Empty => true,
    };
    canonical_host
        && (scheme != "file" || authority.username.is_empty() && authority.password.is_none())
}

fn is_canonical_path(path: &Path) -> bool {
    match path {
        Path::Segments(segments) => segments.iter().all(|segment| {
            !segment.contains('/') && percent_encoding::encode(segment, EncodeSet::Path) == *segment
        }),
        Path::Opaque(_) => true,
    }
}

fn is_canonical_opaque_path(path: &Path) -> bool {
    match path {
        Path::Segments(_) => true,
        Path::Opaque(value) => {
            !value.ends_with(' ') && percent_encoding::encode(value, EncodeSet::C0Control) == *value
        }
    }
}

fn is_canonical_query(value: Option<&str>, special: bool) -> bool {
    value.is_none_or(|value| {
        percent_encoding::encode(
            value,
            if special {
                EncodeSet::SpecialQuery
            } else {
                EncodeSet::Query
            },
        ) == value
    })
}

fn is_canonical_fragment(value: Option<&str>) -> bool {
    value.is_none_or(|value| percent_encoding::encode(value, EncodeSet::Fragment) == value)
}

impl Authority {
    /// Creates authority state that remains tied to a host.
    pub(super) fn new(
        username: String,
        password: Option<String>,
        host: Host,
        port: Option<u16>,
    ) -> Self {
        Self {
            username,
            password,
            host,
            port,
        }
    }
}

fn is_special_scheme(scheme: &str) -> bool {
    matches!(scheme, "ftp" | "file" | "http" | "https" | "ws" | "wss")
}

fn default_port(scheme: &str) -> Option<u16> {
    match scheme {
        "ftp" => Some(21),
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        _ => None,
    }
}

fn is_canonical_scheme(value: &str) -> bool {
    let mut characters = value.bytes();
    matches!(characters.next(), Some(first) if first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, b'+' | b'-' | b'.')
        })
}

fn is_rooted_path(path: &Path) -> bool {
    matches!(path, Path::Segments(segments) if segments.first().is_some_and(String::is_empty))
}

fn is_empty_segments(path: &Path) -> bool {
    matches!(path, Path::Segments(segments) if segments.is_empty())
}

fn is_single_empty_segment_path(path: &Path) -> bool {
    matches!(path, Path::Segments(segments) if segments.len() == 1 && segments[0].is_empty())
}

/// Private record construction failure without user-supplied diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Invalid,
}
