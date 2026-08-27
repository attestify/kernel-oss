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

use super::host::Host;

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
    pub(super) scheme: String,
    pub(super) username: String,
    pub(super) password: Option<String>,
    pub(super) host: Option<Host>,
    pub(super) port: Option<u16>,
    pub(super) path: Path,
    pub(super) query: Option<String>,
    pub(super) fragment: Option<String>,
    pub(super) has_authority: bool,
    pub(super) special: bool,
}

impl Record {
    /// Creates a canonical record while enforcing representation-level invariants.
    #[expect(
        clippy::too_many_arguments,
        reason = "each canonical URL component is independently required to validate private record invariants"
    )]
    pub(super) fn new(
        scheme: String,
        username: String,
        password: Option<String>,
        host: Option<Host>,
        port: Option<u16>,
        path: Path,
        query: Option<String>,
        fragment: Option<String>,
        has_authority: bool,
        special: bool,
    ) -> Result<Self, Error> {
        if scheme.is_empty()
            || (special && matches!(host, Some(Host::Opaque(_))))
            || matches!(path, Path::Opaque(_)) && has_authority
        {
            return Err(Error::Invalid);
        }
        if host.is_none() && (port.is_some() || !username.is_empty() || password.is_some()) {
            return Err(Error::Invalid);
        }
        Ok(Self {
            scheme,
            username,
            password,
            host,
            port,
            path,
            query,
            fragment,
            has_authority,
            special,
        })
    }

    /// Returns whether this record cannot serve as a relative-resolution base.
    pub(super) fn has_opaque_path(&self) -> bool {
        matches!(self.path, Path::Opaque(_))
    }
}

/// Private record construction failure without user-supplied diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Invalid,
}
