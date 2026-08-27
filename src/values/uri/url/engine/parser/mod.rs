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
    record::{self, Path, Record},
};

/// Private, typed nonfatal conditions for the future parser-wide validation model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Validation {
    InvalidUrlUnit,
    SpecialBackslash,
    Credentials,
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
    SpecialAuthoritySlashes,
    SpecialAuthorityIgnoreSlashes,
    Authority,
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
    let (prepared, mut validations) = preprocess(input);
    let (before_fragment, fragment) = split_component(&prepared, '#');
    if fragment.is_some() {
        transition(State::Fragment)?;
    }
    let (before_query, query) = split_component(before_fragment, '?');
    if query.is_some() {
        transition(State::Query)?;
    }
    match split_scheme(before_query) {
        Some((scheme, remainder)) => {
            let scheme = scheme.to_ascii_lowercase();
            if let Some(base) = base
                && base.special
                && base.scheme == scheme
                && !remainder.starts_with("//")
                && !remainder.starts_with('\\')
            {
                resolve_relative(base, remainder, query, fragment, &mut validations)
            } else {
                parse_absolute(scheme, remainder, query, fragment, &mut validations)
            }
        }
        None => {
            transition(State::NoScheme)?;
            let base = base.ok_or(Error::Invalid)?;
            resolve_relative(base, before_query, query, fragment, &mut validations)
        }
    }
}

fn parse_absolute(
    scheme: String,
    remainder: &str,
    query: Option<&str>,
    fragment: Option<&str>,
    validations: &mut Vec<Validation>,
) -> Result<Outcome, Error> {
    transition(State::SchemeStart)?;
    transition(State::Scheme)?;
    let special = is_special(&scheme);
    if special {
        parse_special_absolute(scheme, remainder, query, fragment, validations)
    } else if let Some(authority) = remainder.strip_prefix("//") {
        transition(State::PathOrAuthority)?;
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
        transition(State::PathStart)?;
        let path = parse_segments(remainder, false, false, validations)?;
        finish(
            Record::new(
                scheme,
                String::new(),
                None,
                None,
                None,
                Path::Segments(path),
                encode_query(query, false),
                encode_fragment(fragment),
                false,
                false,
            )?,
            validations,
        )
    } else {
        transition(State::OpaquePath)?;
        let path = encode_opaque_path(remainder, query.is_some() || fragment.is_some());
        finish(
            Record::new(
                scheme,
                String::new(),
                None,
                None,
                None,
                Path::Opaque(path),
                encode_query(query, false),
                encode_fragment(fragment),
                false,
                false,
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
        transition(State::PathOrAuthority)?;
        return parse_file(scheme, remainder, query, fragment, validations);
    }
    transition(State::SpecialAuthoritySlashes)?;
    let normalized = normalize_special_separators(remainder, validations);
    let authority = normalized
        .strip_prefix("//")
        .or_else(|| {
            normalized
                .strip_prefix('/')
                .map(|value| value.trim_start_matches('/'))
        })
        .unwrap_or(&normalized);
    transition(State::SpecialAuthorityIgnoreSlashes)?;
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
    if let Some(authority) = normalized.strip_prefix("//") {
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
            String::new(),
            None,
            Some(Host::Empty),
            None,
            Path::Segments(path),
            encode_query(query, true),
            encode_fragment(fragment),
            true,
            true,
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
    transition(State::Authority)?;
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
                String::new(),
                None,
                Some(Host::Empty),
                None,
                Path::Segments(path),
                encode_query(query, true),
                encode_fragment(fragment),
                true,
                true,
            )?,
            validations,
        );
    }
    let (username, password, host, port) =
        parse_authority(authority, &scheme, special, file, validations)?;
    transition(State::PathStart)?;
    let path = if file {
        parse_file_segments(raw_path, true, validations)?
    } else {
        parse_segments(raw_path, special, true, validations)?
    };
    finish(
        Record::new(
            scheme,
            username,
            password,
            Some(host),
            port,
            Path::Segments(path),
            encode_query(query, special),
            encode_fragment(fragment),
            true,
            special,
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
    transition(State::Relative)?;
    let reference = if base.special {
        normalize_special_separators(reference, validations)
    } else {
        reference.to_owned()
    };
    if let Some(stripped_authority) = reference.strip_prefix("//") {
        transition(State::RelativeSlash)?;
        let authority = if base.special && base.scheme != "file" {
            reference.trim_start_matches('/')
        } else {
            stripped_authority
        };
        return parse_with_authority(
            base.scheme.clone(),
            base.special,
            authority,
            query,
            fragment,
            validations,
            base.scheme == "file",
        );
    }
    if base.scheme == "file" && begins_windows_drive(reference.trim_start_matches('/')) {
        let mut record = base.clone();
        record.path = Path::Segments(parse_file_segments(
            &format!(
                "/{}",
                normalize_windows_drive(reference.trim_start_matches('/'))
            ),
            true,
            validations,
        )?);
        record.query = encode_query(query, true);
        record.fragment = encode_fragment(fragment);
        return finish(record, validations);
    }
    if base.has_opaque_path() {
        if !reference.is_empty() || query.is_some() {
            return Err(Error::Invalid);
        }
        let mut record = base.clone();
        record.fragment = encode_fragment(fragment);
        return finish(record, validations);
    }
    let mut record = base.clone();
    record.fragment = encode_fragment(fragment);
    if reference.is_empty() {
        if query.is_some() {
            record.query = encode_query(query, record.special);
        }
        return finish(record, validations);
    }
    transition(State::Path)?;
    let absolute = reference.starts_with('/');
    let mut segments = match &record.path {
        Path::Segments(segments) => segments.clone(),
        Path::Opaque(_) => return Err(Error::Invalid),
    };
    if absolute {
        let drive = match &record.path {
            Path::Segments(segments) if record.scheme == "file" => segments
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
        segments = if record.scheme == "file" {
            parse_file_segments(&path, true, validations)?
        } else {
            parse_segments(&path, record.special, record.has_authority, validations)?
        };
    } else {
        if segments.is_empty() && record.has_authority {
            segments.push(String::new());
        } else if segments.len() > 1 {
            segments.pop();
        }
        append_relative_segments(
            &mut segments,
            &reference,
            record.scheme == "file",
            record.special,
        );
        if segments.is_empty() && record.has_authority {
            segments.push(String::new());
            segments.push(String::new());
        }
    }
    record.path = Path::Segments(segments);
    record.query = encode_query(query, record.special);
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
            validations.push(Validation::Credentials);
            (Some(parts.0), parts.1)
        });
    let (username, password) = userinfo.map_or((String::new(), None), |value| {
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
    transition(State::PathStart)?;
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
        validations.push(Validation::SpecialBackslash);
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

fn transition(state: State) -> Result<(), Error> {
    match state {
        State::SchemeStart
        | State::Scheme
        | State::NoScheme
        | State::SpecialAuthoritySlashes
        | State::SpecialAuthorityIgnoreSlashes
        | State::Authority
        | State::Relative
        | State::RelativeSlash
        | State::PathOrAuthority
        | State::PathStart
        | State::Path
        | State::OpaquePath
        | State::Query
        | State::Fragment => Ok(()),
    }
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
