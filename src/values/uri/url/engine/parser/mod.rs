//! Private WHATWG-oriented URL parser and relative-reference resolver.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "P4 private parser is wired to the public facade in P5."
    )
)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod wpt_vectors;

use super::{
    host::{self, Host},
    percent_encoding::{self, EncodeSet},
    record::{self, Authority, Path, Record},
};

/// Private, typed nonfatal conditions for the future parser-wide validation model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Validation {
    InvalidUrlUnit,
    SpecialSchemeMissingFollowingSolidus,
    InvalidReverseSolidus,
    InvalidCredentials,
    FileInvalidWindowsDriveLetter,
    FileInvalidWindowsDriveLetterHost,
    Host(host::Validation),
}

/// A successful private parse together with all nonfatal validation conditions.
#[derive(Clone, Eq, PartialEq)]
pub(super) struct Outcome {
    pub(super) record: Record,
    pub(super) validations: Vec<Validation>,
}

/// Private parse failure without supplied text or private-state diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Invalid,
    Overflow,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    SchemeStart,
    Scheme,
    NoScheme,
    SpecialRelativeOrAuthority,
    SpecialAuthoritySlashes,
    SpecialAuthorityIgnoreSlashes,
    Authority,
    Host,
    Hostname,
    Port,
    File,
    FileSlash,
    FileHost,
    Relative,
    RelativeSlash,
    PathOrAuthority,
    PathStart,
    Path,
    OpaquePath,
    Query,
    Fragment,
}

/// Parses one UTF-8 URL input against an optional private base record.
pub(super) fn parse(input: &str, base: Option<&Record>) -> Result<Outcome, Error> {
    Machine::new(input, base).run()
}

/// State-driven private parser dispatcher. Individual states select the relevant
/// parsing operation; helpers only implement the component algorithms selected by
/// this machine.
struct Machine<'a> {
    state: State,
    base: Option<&'a Record>,
    body: String,
    query: Option<String>,
    fragment: Option<String>,
    scheme: Option<String>,
    remainder: String,
    validations: Vec<Validation>,
    authority: Option<AuthorityWork>,
}

struct AuthorityWork {
    scheme: String,
    special: bool,
    raw_path: String,
    username: String,
    password: Option<String>,
    host_text: String,
    raw_port: Option<String>,
    port: Option<u16>,
    bracketed: bool,
    host: Option<Host>,
}

impl<'a> Machine<'a> {
    fn new(input: &str, base: Option<&'a Record>) -> Self {
        let (body, validations) = preprocess(input);
        Self {
            state: State::Fragment,
            base,
            body,
            query: None,
            fragment: None,
            scheme: None,
            remainder: String::new(),
            validations,
            authority: None,
        }
    }

    fn run(mut self) -> Result<Outcome, Error> {
        loop {
            match self.state {
                State::Fragment => {
                    let (before, fragment) = split_component(&self.body, '#');
                    let body = before.to_owned();
                    let fragment = fragment.map(str::to_owned);
                    self.body = body;
                    self.fragment = fragment;
                    if let Some(fragment) = self.fragment.as_deref() {
                        validate_component(fragment, &mut self.validations);
                    }
                    self.state = State::Query;
                }
                State::Query => {
                    let (before, query) = split_component(&self.body, '?');
                    let body = before.to_owned();
                    let query = query.map(str::to_owned);
                    self.body = body;
                    self.query = query;
                    if let Some(query) = self.query.as_deref() {
                        validate_component(query, &mut self.validations);
                    }
                    self.state = State::SchemeStart;
                }
                State::SchemeStart => match split_scheme(&self.body) {
                    Some((scheme, remainder)) => {
                        self.scheme = Some(scheme.to_ascii_lowercase());
                        self.remainder = remainder.to_owned();
                        self.state = State::Scheme;
                    }
                    None => self.state = State::NoScheme,
                },
                State::Scheme => {
                    let scheme = self.scheme.as_deref().ok_or(Error::Invalid)?;
                    if let Some(base) = self.base
                        && base.is_special()
                        && base.scheme() == scheme
                        && !self.remainder.starts_with("//")
                        && !self.remainder.starts_with('\\')
                    {
                        self.state = State::SpecialRelativeOrAuthority;
                    } else if scheme == "file" {
                        self.state = State::File;
                    } else if is_special(scheme) {
                        if !self.remainder.starts_with("//") && !self.remainder.starts_with('\\') {
                            self.validations
                                .push(Validation::SpecialSchemeMissingFollowingSolidus);
                        }
                        self.state = State::SpecialAuthoritySlashes;
                    } else if self.remainder.starts_with("//") {
                        self.state = State::PathOrAuthority;
                    } else if self.remainder.starts_with('/') {
                        self.state = State::PathStart;
                    } else {
                        self.state = State::OpaquePath;
                    }
                }
                State::SpecialRelativeOrAuthority => {
                    let base = self.base.ok_or(Error::Invalid)?;
                    return resolve_relative(
                        base,
                        &self.remainder,
                        self.query.as_deref(),
                        self.fragment.as_deref(),
                        &mut self.validations,
                    );
                }
                State::NoScheme => {
                    self.state = State::Relative;
                }
                State::Relative => {
                    if self.base.is_some_and(Record::is_special) {
                        self.body = normalize_special_separators(&self.body, &mut self.validations);
                    }
                    if self.body.starts_with("//") {
                        self.state = State::RelativeSlash;
                    } else {
                        let base = self.base.ok_or(Error::Invalid)?;
                        return resolve_relative(
                            base,
                            &self.body,
                            self.query.as_deref(),
                            self.fragment.as_deref(),
                            &mut self.validations,
                        );
                    }
                }
                State::RelativeSlash => {
                    let base = self.base.ok_or(Error::Invalid)?;
                    let reference = if base.is_special() && !base.is_file() {
                        self.body.trim_start_matches('/')
                    } else {
                        self.body.strip_prefix("//").ok_or(Error::Invalid)?
                    };
                    if base.is_file() {
                        return parse_file(
                            base.scheme().to_owned(),
                            &self.body,
                            self.query.as_deref(),
                            self.fragment.as_deref(),
                            &mut self.validations,
                        );
                    }
                    self.scheme = Some(base.scheme().to_owned());
                    self.remainder = reference.to_owned();
                    self.state = State::Authority;
                }
                State::SpecialAuthoritySlashes => {
                    self.remainder =
                        normalize_special_separators(&self.remainder, &mut self.validations);
                    self.state = State::SpecialAuthorityIgnoreSlashes;
                }
                State::SpecialAuthorityIgnoreSlashes => {
                    self.remainder = self.remainder.trim_start_matches('/').to_owned();
                    self.state = State::Authority;
                }
                State::PathOrAuthority => {
                    self.remainder = self
                        .remainder
                        .strip_prefix("//")
                        .ok_or(Error::Invalid)?
                        .to_owned();
                    self.state = State::Authority;
                }
                State::Authority => {
                    self.begin_authority()?;
                    self.state = State::Host;
                }
                State::Host => {
                    let authority = self.authority.as_mut().ok_or(Error::Invalid)?;
                    if authority.bracketed {
                        authority.host = Some(host::parse_ipv6(&authority.host_text)?);
                        self.state = State::Port;
                    } else {
                        self.state = State::Hostname;
                    }
                }
                State::Hostname => {
                    self.parse_hostname()?;
                    self.state = State::Port;
                }
                State::Port => {
                    self.parse_authority_port()?;
                    self.state = State::Path;
                }
                State::File => {
                    self.state = State::FileSlash;
                }
                State::FileSlash => {
                    self.state = State::FileHost;
                }
                State::FileHost => {
                    let scheme = self.scheme.take().ok_or(Error::Invalid)?;
                    return parse_file(
                        scheme,
                        &self.remainder,
                        self.query.as_deref(),
                        self.fragment.as_deref(),
                        &mut self.validations,
                    );
                }
                State::PathStart => {
                    let scheme = self.scheme.take().ok_or(Error::Invalid)?;
                    return parse_absolute(
                        scheme,
                        &self.remainder,
                        self.query.as_deref(),
                        self.fragment.as_deref(),
                        &mut self.validations,
                    );
                }
                State::OpaquePath => {
                    let scheme = self.scheme.take().ok_or(Error::Invalid)?;
                    return parse_absolute(
                        scheme,
                        &self.remainder,
                        self.query.as_deref(),
                        self.fragment.as_deref(),
                        &mut self.validations,
                    );
                }
                State::Path => return self.finish_authority_path(),
            }
        }
    }

    fn begin_authority(&mut self) -> Result<(), Error> {
        let scheme = self.scheme.take().ok_or(Error::Invalid)?;
        let special = is_special(&scheme);
        let separator = self
            .remainder
            .char_indices()
            .find_map(|(index, character)| {
                (character == '/' || special && character == '\\').then_some(index)
            });
        let (authority, raw_path) = separator.map_or((self.remainder.as_str(), ""), |index| {
            self.remainder.split_at(index)
        });
        let (userinfo, host_port) = authority
            .rsplit_once('@')
            .map_or((None, authority), |parts| {
                self.validations.push(Validation::InvalidCredentials);
                (Some(parts.0), parts.1)
            });
        let (username, password) = userinfo.map_or((String::new(), None), |value| {
            validate_component(value, &mut self.validations);
            let value = value.replace('@', "%40");
            let (username, password) = value
                .split_once(':')
                .map_or((value.as_str(), None), |parts| (parts.0, Some(parts.1)));
            (
                percent_encoding::encode(username, EncodeSet::UserInfo),
                password.map(|password| percent_encoding::encode(password, EncodeSet::UserInfo)),
            )
        });
        let (host_text, port_text, bracketed) = split_host_port(host_port)?;
        if host_text.is_empty() && (special || userinfo.is_some() || port_text.is_some()) {
            return Err(Error::Invalid);
        }
        self.authority = Some(AuthorityWork {
            scheme,
            special,
            raw_path: raw_path.to_owned(),
            username,
            password,
            host_text: host_text.to_owned(),
            raw_port: port_text.map(str::to_owned),
            port: None,
            bracketed,
            host: None,
        });
        Ok(())
    }

    fn parse_hostname(&mut self) -> Result<(), Error> {
        let authority = self.authority.as_mut().ok_or(Error::Invalid)?;
        authority.host = Some(if authority.host_text.is_empty() {
            Host::Empty
        } else if authority.special {
            let outcome = host::parse_special(&authority.host_text)?;
            self.validations
                .extend(outcome.validations.into_iter().map(Validation::Host));
            outcome.value
        } else {
            let outcome = host::parse_opaque(&authority.host_text)?;
            self.validations
                .extend(outcome.validations.into_iter().map(Validation::Host));
            outcome.value
        });
        Ok(())
    }

    fn parse_authority_port(&mut self) -> Result<(), Error> {
        let authority = self.authority.as_mut().ok_or(Error::Invalid)?;
        let port = parse_port(
            authority.raw_port.as_deref(),
            &authority.scheme,
            &authority.host_text,
            authority.special,
            false,
            &mut self.validations,
        )?;
        let host = authority.host.take().ok_or(Error::Invalid)?;
        authority.host = Some(host);
        authority.port = port;
        Ok(())
    }

    fn finish_authority_path(&mut self) -> Result<Outcome, Error> {
        let authority = self.authority.take().ok_or(Error::Invalid)?;
        let host = authority.host.ok_or(Error::Invalid)?;
        let path = parse_segments(
            &authority.raw_path,
            authority.special,
            true,
            &mut self.validations,
        )?;
        finish(
            Record::new(
                authority.scheme,
                Some(Authority::new(
                    authority.username,
                    authority.password,
                    host,
                    authority.port,
                )),
                Path::Segments(path),
                encode_query(self.query.as_deref(), authority.special),
                encode_fragment(self.fragment.as_deref()),
            )?,
            &mut self.validations,
        )
    }
}

fn parse_absolute(
    scheme: String,
    remainder: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
) -> Result<Outcome, Error> {
    let special = is_special(&scheme);
    if special {
        parse_special_absolute(scheme, remainder, query, fragment, validations)
    } else if let Some(authority) = remainder.strip_prefix("//") {
        parse_with_authority(
            scheme,
            false,
            authority,
            query,
            fragment,
            validations,
            false,
        )
    } else if remainder.starts_with('/') {
        let path = parse_segments(remainder, false, false, validations)?;
        finish(
            Record::new(
                scheme,
                None,
                Path::Segments(path),
                encode_query(query, false),
                encode_fragment(fragment),
            )?,
            validations,
        )
    } else {
        validate_component(remainder, validations);
        let path = encode_opaque_path(remainder, query.is_some() || fragment.is_some());
        finish(
            Record::new(
                scheme,
                None,
                Path::Opaque(path),
                encode_query(query, false),
                encode_fragment(fragment),
            )?,
            validations,
        )
    }
}

fn parse_special_absolute(
    scheme: String,
    remainder: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
) -> Result<Outcome, Error> {
    if scheme == "file" {
        return parse_file(scheme, remainder, query, fragment, validations);
    }
    let normalized = normalize_special_separators(remainder, validations);
    let authority = normalized
        .strip_prefix("//")
        .or_else(|| {
            normalized
                .strip_prefix('/')
                .map(|value| value.trim_start_matches('/'))
        })
        .unwrap_or(&normalized);
    parse_with_authority(scheme, true, authority, query, fragment, validations, false)
}

fn parse_file(
    scheme: String,
    remainder: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
) -> Result<Outcome, Error> {
    let normalized = normalize_special_separators(remainder, validations);
    if normalized.contains('|') {
        validations.push(Validation::FileInvalidWindowsDriveLetter);
    }
    if let Some(authority) = normalized.strip_prefix("//") {
        if begins_windows_drive(authority) {
            validations.push(Validation::FileInvalidWindowsDriveLetterHost);
        }
        return parse_with_authority(scheme, true, authority, query, fragment, validations, true);
    }
    let normalized = if normalized.starts_with('/') {
        normalized
    } else {
        format!("/{normalized}")
    };
    let normalized = if begins_windows_drive(normalized.trim_start_matches('/')) {
        format!(
            "/{}",
            normalize_windows_drive(normalized.trim_start_matches('/'))
        )
    } else {
        normalized
    };
    let path = parse_file_segments(&normalized, true, validations)?;
    finish(
        Record::new(
            scheme,
            Some(Authority::new(String::new(), None, Host::Empty, None)),
            Path::Segments(path),
            encode_query(query, true),
            encode_fragment(fragment),
        )?,
        validations,
    )
}

fn parse_with_authority(
    scheme: String,
    special: bool,
    authority_and_path: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
    file: bool,
) -> Result<Outcome, Error> {
    let separator = authority_and_path
        .char_indices()
        .find_map(|(index, character)| {
            (character == '/' || (special && character == '\\')).then_some(index)
        });
    let (authority, raw_path) = separator.map_or((authority_and_path, ""), |index| {
        authority_and_path.split_at(index)
    });
    if file && begins_windows_drive(authority) {
        let path = parse_file_segments(
            &format!("/{}{}", normalize_windows_drive(authority), raw_path),
            true,
            validations,
        )?;
        return finish(
            Record::new(
                scheme,
                Some(Authority::new(String::new(), None, Host::Empty, None)),
                Path::Segments(path),
                encode_query(query, true),
                encode_fragment(fragment),
            )?,
            validations,
        );
    }
    let (username, password, host, port) =
        parse_authority(authority, &scheme, special, file, validations)?;
    let path = if file {
        parse_file_segments(raw_path, true, validations)?
    } else {
        parse_segments(raw_path, special, true, validations)?
    };
    finish(
        Record::new(
            scheme,
            Some(Authority::new(username, password, host, port)),
            Path::Segments(path),
            encode_query(query, special),
            encode_fragment(fragment),
        )?,
        validations,
    )
}

fn resolve_relative(
    base: &Record,
    reference: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
) -> Result<Outcome, Error> {
    let reference = if base.is_special() {
        normalize_special_separators(reference, validations)
    } else {
        reference.to_owned()
    };
    if let Some(stripped_authority) = reference.strip_prefix("//") {
        let authority = if base.is_special() && !base.is_file() {
            reference.trim_start_matches('/')
        } else {
            stripped_authority
        };
        return parse_with_authority(
            base.scheme().to_owned(),
            base.is_special(),
            authority,
            query,
            fragment,
            validations,
            base.is_file(),
        );
    }
    if base.is_file() && begins_windows_drive(reference.trim_start_matches('/')) {
        let record = base.with_path(Path::Segments(parse_file_segments(
            &format!(
                "/{}",
                normalize_windows_drive(reference.trim_start_matches('/'))
            ),
            true,
            validations,
        )?))?;
        let record = record.with_query(encode_query(query, true))?;
        let record = record.with_fragment(encode_fragment(fragment))?;
        return finish(record, validations);
    }
    if base.has_opaque_path() {
        if !reference.is_empty() || query.is_some() {
            return Err(Error::Invalid);
        }
        let record = base.with_fragment(encode_fragment(fragment))?;
        return finish(record, validations);
    }
    let record = base.with_fragment(encode_fragment(fragment))?;
    if reference.is_empty() {
        if query.is_some() {
            return finish(
                record.with_query(encode_query(query, record.is_special()))?,
                validations,
            );
        }
        return finish(record, validations);
    }
    let absolute = reference.starts_with('/');
    let mut segments = match record.path() {
        Path::Segments(segments) => segments.clone(),
        Path::Opaque(_) => return Err(Error::Invalid),
    };
    if absolute {
        let drive = match record.path() {
            Path::Segments(segments) if record.is_file() => segments
                .get(1)
                .filter(|value| is_windows_drive_segment(value)),
            Path::Segments(_) | Path::Opaque(_) => None,
        };
        let path = if let Some(drive) = drive {
            let suffix = reference.trim_start_matches('/');
            if suffix.is_empty() {
                format!("/{drive}/")
            } else {
                format!("/{drive}/{suffix}")
            }
        } else {
            reference.clone()
        };
        segments = if record.is_file() {
            parse_file_segments(&path, true, validations)?
        } else {
            parse_segments(
                &path,
                record.is_special(),
                record.has_authority(),
                validations,
            )?
        };
    } else {
        if segments.is_empty() && record.has_authority() {
            segments.push(String::new());
        } else if segments.len() > 1 {
            segments.pop();
        }
        append_relative_segments(
            &mut segments,
            &reference,
            record.is_file(),
            record.is_special(),
        );
        if segments.is_empty() && record.has_authority() {
            segments.push(String::new());
            segments.push(String::new());
        }
    }
    let record = record.with_path(Path::Segments(segments))?;
    let record = record.with_query(encode_query(query, record.is_special()))?;
    finish(record, validations)
}

fn parse_authority(
    authority: &str,
    scheme: &str,
    special: bool,
    file: bool,
    validations: &mut Vec<Validation>,
) -> Result<(String, Option<String>, Host, Option<u16>), Error> {
    let (userinfo, host_port) = authority
        .rsplit_once('@')
        .map_or((None, authority), |parts| {
            validations.push(Validation::InvalidCredentials);
            (Some(parts.0), parts.1)
        });
    let (username, password) = userinfo.map_or((String::new(), None), |value| {
        validate_component(value, validations);
        let value = value.replace('@', "%40");
        let (username, password) = value
            .split_once(':')
            .map_or((value.as_str(), None), |parts| (parts.0, Some(parts.1)));
        (
            percent_encoding::encode(username, EncodeSet::UserInfo),
            password.map(|password| percent_encoding::encode(password, EncodeSet::UserInfo)),
        )
    });
    let (host_text, port_text, bracketed) = split_host_port(host_port)?;
    if host_text.is_empty() && (special && !file || userinfo.is_some() || port_text.is_some()) {
        return Err(Error::Invalid);
    }
    let host = if host_text.is_empty() {
        Host::Empty
    } else if bracketed {
        host::parse_ipv6(host_text)?
    } else if special {
        let outcome = host::parse_special(host_text)?;
        validations.extend(outcome.validations.into_iter().map(Validation::Host));
        if file && matches!(outcome.value, Host::Domain(ref value) if value == "localhost") {
            Host::Empty
        } else {
            outcome.value
        }
    } else {
        let outcome = host::parse_opaque(host_text)?;
        validations.extend(outcome.validations.into_iter().map(Validation::Host));
        outcome.value
    };
    let port = parse_port(port_text, scheme, host_text, special, file, validations)?;
    Ok((username, password, host, port))
}

fn split_host_port(authority: &str) -> Result<(&str, Option<&str>, bool), Error> {
    if let Some(rest) = authority.strip_prefix('[') {
        let end = rest.find(']').ok_or(Error::Invalid)?;
        let host = &rest[..end];
        let after = &rest[end + 1..];
        return if after.is_empty() {
            Ok((host, None, true))
        } else {
            Ok((
                host,
                Some(after.strip_prefix(':').ok_or(Error::Invalid)?),
                true,
            ))
        };
    }
    Ok(authority
        .rsplit_once(':')
        .map_or((authority, None, false), |parts| {
            (parts.0, Some(parts.1), false)
        }))
}

fn parse_port(
    port: Option<&str>,
    scheme: &str,
    host: &str,
    special: bool,
    file: bool,
    _validations: &mut Vec<Validation>,
) -> Result<Option<u16>, Error> {
    let Some(port) = port else {
        return Ok(None);
    };
    if file || host.is_empty() && port.is_empty() {
        return if port.is_empty() {
            Ok(None)
        } else {
            Err(Error::Invalid)
        };
    }
    if port.is_empty() {
        return Ok(None);
    }
    let mut value = 0u32;
    for byte in port.bytes() {
        if !byte.is_ascii_digit() {
            return Err(Error::Invalid);
        }
        value = value
            .checked_mul(10)
            .and_then(|current| current.checked_add(u32::from(byte - b'0')))
            .ok_or(Error::Overflow)?;
        if value > u32::from(u16::MAX) {
            return Err(Error::Invalid);
        }
    }
    let value = u16::try_from(value).map_err(|_| Error::Overflow)?;
    if special && default_port(scheme) == Some(value) {
        return Ok(None);
    }
    Ok(Some(value))
}

fn parse_segments(
    raw: &str,
    special: bool,
    authority: bool,
    validations: &mut Vec<Validation>,
) -> Result<Vec<String>, Error> {
    parse_segments_inner(raw, special, authority, false, validations)
}

fn parse_file_segments(
    raw: &str,
    authority: bool,
    validations: &mut Vec<Validation>,
) -> Result<Vec<String>, Error> {
    parse_segments_inner(raw, true, authority, true, validations)
}

fn parse_segments_inner(
    raw: &str,
    special: bool,
    authority: bool,
    preserve_drive: bool,
    validations: &mut Vec<Validation>,
) -> Result<Vec<String>, Error> {
    validate_component(raw, validations);
    let raw = if special {
        normalize_special_separators(raw, validations)
    } else {
        raw.to_owned()
    };
    let rooted = raw.starts_with('/');
    let mut output = if rooted {
        vec![String::new()]
    } else {
        Vec::new()
    };
    let content = raw.strip_prefix('/').unwrap_or(&raw);
    if content.is_empty() {
        if authority && !special && !rooted {
            return Ok(output);
        }
        if authority && output.is_empty() {
            output.push(String::new());
        }
        if authority || rooted {
            output.push(String::new());
        }
        return Ok(output);
    }
    if !special && !authority {
        let segments: Vec<&str> = content.split('/').collect();
        for (index, segment) in segments.iter().enumerate() {
            if is_double_dot(segment) {
                if output.len() > usize::from(rooted) {
                    output.pop();
                    output.push(".".to_owned());
                } else if segments.get(index + 2).is_some() {
                    // The non-special path state retains a leading dot segment
                    // when a root-level double-dot is followed by a second
                    // separator. This preserves the distinction between `/../`
                    // and `/..//` in the URL record.
                    output.push(".".to_owned());
                }
            } else {
                output.push(percent_encoding::encode(segment, EncodeSet::Path));
            }
        }
        return Ok(output);
    }
    for raw_segment in content.split('/') {
        let normalized_segment;
        let raw_segment =
            if special && rooted && output.len() == 1 && begins_windows_drive(raw_segment) {
                normalized_segment = normalize_windows_drive(raw_segment);
                &normalized_segment
            } else {
                raw_segment
            };
        if is_double_dot(raw_segment) {
            let drive_root = preserve_drive
                && output.len() == 2
                && output
                    .get(1)
                    .is_some_and(|value| is_windows_drive_segment(value));
            if output.len() > usize::from(rooted) && !drive_root {
                output.pop();
            }
            continue;
        }
        if is_single_dot(raw_segment) {
            continue;
        }
        output.push(percent_encoding::encode(raw_segment, EncodeSet::Path));
    }
    let terminal = content.rsplit('/').next().unwrap_or_default();
    if is_single_dot(terminal) || is_double_dot(terminal) {
        output.push(String::new());
    }
    if output.is_empty() && authority {
        output.push(String::new());
        output.push(String::new());
    }
    Ok(output)
}

fn append_relative_segments(target: &mut Vec<String>, raw: &str, file: bool, special: bool) {
    let raw_segments: Vec<&str> = raw.split('/').collect();
    for (index, segment) in raw_segments.iter().enumerate() {
        if is_double_dot(segment) {
            let drive_root = file
                && target.len() == 2
                && target
                    .get(1)
                    .is_some_and(|value| is_windows_drive_segment(value));
            if target.len() > 1 && !drive_root {
                target.pop();
                if !special && target.last().is_some_and(|value| value == ".") {
                    target.pop();
                }
            }
            if !special
                && target.len() <= 1
                && raw_segments.get(index + 1) == Some(&"")
                && raw_segments.get(index + 2).is_some()
            {
                target.push(".".to_owned());
            }
        } else if !is_single_dot(segment) {
            target.push(percent_encoding::encode(segment, EncodeSet::Path));
        }
    }
    let terminal = raw.rsplit('/').next().unwrap_or_default();
    if is_single_dot(terminal) || is_double_dot(terminal) {
        target.push(String::new());
    }
}

fn is_single_dot(segment: &str) -> bool {
    segment.eq_ignore_ascii_case(".") || segment.eq_ignore_ascii_case("%2e")
}

fn is_double_dot(segment: &str) -> bool {
    ["..", ".%2e", "%2e.", "%2e%2e"]
        .iter()
        .any(|value| segment.eq_ignore_ascii_case(value))
}

fn encode_query(query: Option<&str>, special: bool) -> Option<String> {
    query.map(|value| {
        percent_encoding::encode(
            value,
            if special {
                EncodeSet::SpecialQuery
            } else {
                EncodeSet::Query
            },
        )
    })
}

fn encode_fragment(fragment: Option<&str>) -> Option<String> {
    fragment.map(|value| percent_encoding::encode(value, EncodeSet::Fragment))
}

fn encode_opaque_path(path: &str, has_query_or_fragment: bool) -> String {
    if has_query_or_fragment && let Some(prefix) = path.strip_suffix(' ') {
        return format!(
            "{}%20",
            percent_encoding::encode(prefix, EncodeSet::C0Control)
        );
    }
    percent_encoding::encode(path, EncodeSet::C0Control)
}

fn validate_component(value: &str, validations: &mut Vec<Validation>) {
    if value.chars().any(is_invalid_url_unit) || has_malformed_percent_triplet(value) {
        validations.push(Validation::InvalidUrlUnit);
    }
}

fn is_invalid_url_unit(character: char) -> bool {
    character <= '\u{001f}' || character == '\u{007f}'
}

fn has_malformed_percent_triplet(input: &str) -> bool {
    let bytes = input.as_bytes();
    bytes.iter().enumerate().any(|(index, byte)| {
        *byte == b'%'
            && !matches!(
                (bytes.get(index + 1), bytes.get(index + 2)),
                (Some(high), Some(low)) if high.is_ascii_hexdigit() && low.is_ascii_hexdigit()
            )
    })
}

fn split_component(input: &str, delimiter: char) -> (&str, Option<&str>) {
    input
        .split_once(delimiter)
        .map_or((input, None), |parts| (parts.0, Some(parts.1)))
}

fn split_scheme(input: &str) -> Option<(&str, &str)> {
    let colon = input.find(':')?;
    let candidate = &input[..colon];
    let mut characters = candidate.chars();
    let first = characters.next()?;
    if !first.is_ascii_alphabetic()
        || !characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
    {
        return None;
    }
    Some((candidate, &input[colon + 1..]))
}

fn preprocess(input: &str) -> (String, Vec<Validation>) {
    let trimmed = input.trim_matches(|character: char| character <= '\u{0020}');
    let mut validations = Vec::new();
    if trimmed.len() != input.len() {
        validations.push(Validation::InvalidUrlUnit);
    }
    let prepared: String = trimmed
        .chars()
        .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
        .collect();
    if prepared.len() != trimmed.len() {
        validations.push(Validation::InvalidUrlUnit);
    }
    (prepared, validations)
}

fn normalize_special_separators(input: &str, validations: &mut Vec<Validation>) -> String {
    if input.contains('\\') {
        validations.push(Validation::InvalidReverseSolidus);
    }
    input.replace('\\', "/")
}

fn is_special(scheme: &str) -> bool {
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

fn begins_windows_drive(value: &str) -> bool {
    let bytes = value.as_bytes();
    matches!(
        bytes,
        [letter, b':' | b'|'] | [letter, b':' | b'|', b'/' | b'\\', ..]
            if letter.is_ascii_alphabetic()
    )
}

fn normalize_windows_drive(value: &str) -> String {
    let mut output = value.to_owned();
    if output.as_bytes().get(1) == Some(&b'|') {
        output.replace_range(1..2, ":");
    }
    output
}

fn is_windows_drive_segment(value: &str) -> bool {
    matches!(value.as_bytes(), [letter, b':'] if letter.is_ascii_alphabetic())
}

fn finish(record: Record, validations: &mut Vec<Validation>) -> Result<Outcome, Error> {
    Ok(Outcome {
        record,
        validations: core::mem::take(validations),
    })
}

impl From<host::Error> for Error {
    fn from(_: host::Error) -> Self {
        Self::Invalid
    }
}

impl From<record::Error> for Error {
    fn from(_: record::Error) -> Self {
        Self::Invalid
    }
}
