//! Private canonical URL serialization.

#[cfg(test)]
mod tests;

use super::{
    host,
    record::{Path, Record},
};

/// Serializes one private canonical URL record without interpreting components again.
pub(in crate::values::uri::url) fn serialize(record: &Record) -> String {
    let mut output = String::new();
    output.push_str(record.scheme());
    output.push(':');
    if record.has_authority() {
        output.push_str("//");
        if !record.username().is_empty() || record.password().is_some() {
            output.push_str(record.username());
            if let Some(password) = record.password() {
                output.push(':');
                output.push_str(password);
            }
            output.push('@');
        }
        if let Some(host) = record.host() {
            output.push_str(&host::serialize(host));
        }
        if let Some(port) = record.port() {
            output.push(':');
            output.push_str(&port.to_string());
        }
    }
    match record.path() {
        Path::Segments(segments) => output.push_str(&segments.join("/")),
        Path::Opaque(path) => output.push_str(path),
    }
    if let Some(query) = record.query() {
        output.push('?');
        output.push_str(query);
    }
    if let Some(fragment) = record.fragment() {
        output.push('#');
        output.push_str(fragment);
    }
    output
}
