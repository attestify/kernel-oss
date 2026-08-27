//! Private URL-engine building blocks; P2 exposes no public URL behavior.

#[cfg(test)]
mod tests;

pub(super) mod form_urlencoded;
pub(super) mod host;
pub(super) mod idna;
pub(super) mod percent_encoding;
