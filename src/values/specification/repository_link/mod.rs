//! Repository link value and builder.

use crate::error::{Audience, Error, Kind};
use crate::values::uri::url::URL;
use std::fmt;

/// The [`RepositoryLink`] value is an NAPE-specific value for capturing the URL for the location of a procedure specification.
///
/// # Assumptions
///  * This defaults all schemes to **git://** if a scheme is not provided.
///  * This allows url inputs values such as **localhost** to be valid.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct RepositoryLink {
    /// Parsed repository URL.
    url: URL,
}

impl fmt::Display for RepositoryLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RepositoryLink([REDACTED])")
    }
}

impl fmt::Debug for RepositoryLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RepositoryLink([REDACTED])")
    }
}

impl RepositoryLink {
    /// Starts a repository link builder.
    pub fn builder() -> RepositoryLinkBuilder {
        RepositoryLinkBuilder::default()
    }

    /// Returns the parsed URL.
    pub fn url(&self) -> &URL {
        &self.url
    }
}

/// Builds a [`RepositoryLink`].
#[derive(Debug, Default, Clone)]
pub struct RepositoryLinkBuilder {
    /// Allowed URL schemes.
    allowed_schema: Vec<String>,
    /// Default URL scheme.
    default_scheme: Option<String>,
    /// Repository link text.
    repo_link: Option<String>,
}

impl RepositoryLinkBuilder {
    /// Sets the list of allowed URL schemes.
    pub fn allowed_schema(mut self, schema: Vec<String>) -> Self {
        self.allowed_schema = schema;
        self
    }

    /// Sets the default URL scheme to apply when one is missing.
    pub fn default_scheme(mut self, scheme: impl Into<String>) -> Self {
        self.default_scheme = Some(scheme.into());
        self
    }

    /// Sets the repository link text to parse and validate.
    pub fn repo_link(mut self, link: impl Into<String>) -> Self {
        self.repo_link = Some(link.into());
        self
    }

    /// Validates the builder and creates a repository link.
    pub fn build(&mut self) -> Result<RepositoryLink, Error> {
        let allowed_schema = self.verify_allowed_schema()?;
        let default_schema = self.verify_default_scheme(&allowed_schema)?;
        let repo_link = self.verify_repo_link(&allowed_schema, &default_schema)?;
        let url = URL::try_new(&repo_link).map_err(|_| repository_error(Audience::System))?;

        Ok(RepositoryLink { url })
    }

    fn verify_allowed_schema(&self) -> Result<Vec<String>, Error> {
        if self.allowed_schema.is_empty() {
            return Err(repository_error(Audience::System));
        }
        Ok(self.allowed_schema.clone())
    }

    fn verify_default_scheme(&mut self, allowed_schema: &[String]) -> Result<String, Error> {
        let default_scheme = self
            .default_scheme
            .take()
            .ok_or_else(|| repository_error(Audience::User))?;

        if default_scheme.trim().is_empty() {
            return Err(repository_error(Audience::System));
        }

        if !allowed_schema.contains(&default_scheme) {
            return Err(repository_error(Audience::System));
        }
        Ok(default_scheme)
    }

    fn verify_repo_link(
        &mut self,
        allowed_schema: &[String],
        default_schema: &str,
    ) -> Result<String, Error> {
        let existing_link = self
            .repo_link
            .take()
            .ok_or_else(|| repository_error(Audience::System))?;

        if existing_link.trim().is_empty() {
            return Err(repository_error(Audience::System));
        }

        self.verify_repo_link_not_malformed(&existing_link)?;

        let repo_link_with_scheme =
            self.apply_default_scheme_to_repo_link(&existing_link, default_schema);

        self.verify_repo_link_scheme_is_allowed(repo_link_with_scheme.as_str(), allowed_schema)?;

        Ok(repo_link_with_scheme)
    }

    fn verify_repo_link_not_malformed(&self, repo_link: &str) -> Result<(), Error> {
        // Check if the link starts with an alphanumeric character (assume host without scheme)
        if repo_link
            .chars()
            .next()
            .is_some_and(|c| c.is_alphanumeric())
        {
            // Proceed to apply default scheme
            Ok(())
        } else {
            // Must contain a proper scheme separator '://'
            let parts: Vec<&str> = repo_link.split("://").collect();
            if parts.len() != 2 || parts[1].is_empty() {
                return Err(repository_error(Audience::System));
            }
            Ok(())
        }
    }

    fn apply_default_scheme_to_repo_link(&self, repo_link: &str, default_scheme: &str) -> String {
        if repo_link.contains("://") {
            let parts: Vec<&str> = repo_link.split("://").collect();
            if parts[0].is_empty() {
                // Starts with ://, prepend default scheme
                format!("{}://{}", default_scheme, parts[1])
            } else {
                // Has a scheme, use as-is
                repo_link.to_string()
            }
        } else {
            // No ://, prepend default scheme
            format!("{}://{}", default_scheme, repo_link)
        }
    }

    fn verify_repo_link_scheme_is_allowed(
        &self,
        verified_repo_link: &str,
        allowed_schemes: &[String],
    ) -> Result<(), Error> {
        let repo_link_schema = self.extract_repo_link_schema(verified_repo_link);
        if allowed_schemes.contains(&repo_link_schema) {
            Ok(())
        } else {
            Err(repository_error(Audience::System))
        }
    }

    fn extract_repo_link_schema(&self, link: &str) -> String {
        let url_parts: Vec<&str> = link.split("://").collect();
        url_parts[0].to_string()
    }
}

fn repository_error(audience: Audience) -> Error {
    Error::new(
        audience,
        Kind::InvalidInput,
        "Repository link validation failed.",
    )
}

#[cfg(test)]
mod tests;
