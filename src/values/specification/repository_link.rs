use crate::error::{Error, Kind};
use crate::values::uri::url::URL;
use std::fmt;

/// The [`RepositoryLink`] value is an NAPE-specific value for capturing the URL for the location of a procedure specification.
///
/// # Assumptions
///  * This defaults all schemes to **git://** if a scheme is not provided.
///  * This allows url inputs values such as **localhost** to be valid.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RepositoryLink {
    url: URL,
}

impl fmt::Display for RepositoryLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url.value.value())
    }
}

impl RepositoryLink {
    pub fn builder() -> RepositoryLinkBuilder {
        RepositoryLinkBuilder::default()
    }

    pub fn url(&self) -> &URL {
        &self.url
    }
}

#[derive(Debug, Default, Clone)]
pub struct RepositoryLinkBuilder {
    allowed_schema: Vec<String>,
    default_scheme: Option<String>,
    repo_link: Option<String>,
}

impl RepositoryLinkBuilder {
    pub fn allowed_schema(mut self, schema: Vec<String>) -> Self {
        self.allowed_schema = schema;
        self
    }

    pub fn default_scheme(mut self, scheme: impl Into<String>) -> Self {
        self.default_scheme = Some(scheme.into());
        self
    }

    pub fn repo_link(mut self, link: impl Into<String>) -> Self {
        self.repo_link = Some(link.into());
        self
    }

    pub fn build(&mut self) -> Result<RepositoryLink, Error> {
        let allowed_schema = self.verify_allowed_schema()?;
        let default_schema = self.verify_default_scheme(&allowed_schema)?;
        let repo_link = self.verify_repo_link(&allowed_schema, &default_schema)?;
        let url = URL::new(&repo_link).map_err(|error| {
            Error::for_system(
                Kind::InvalidInput,
                format!(
                    "The provided repository link [{}] is malformed. {}",
                    &repo_link, error
                ),
            )
        })?;

        Ok(RepositoryLink { url })
    }

    fn verify_allowed_schema(&self) -> Result<Vec<String>, Error> {
        if self.allowed_schema.is_empty() {
            return Err(Error::for_system(
                Kind::InvalidInput,
                "No allowed schemes were provided, please provide at least one allowed scheme.",
            ));
        }
        Ok(self.allowed_schema.clone())
    }

    fn verify_default_scheme(&mut self, allowed_schema: &Vec<String>) -> Result<String, Error> {
        let default_scheme = self.default_scheme.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A default scheme was not provided. Please provide a default scheme.",
            )
        })?;

        if default_scheme.trim().is_empty() {
            return Err(Error::for_system(
                Kind::InvalidInput,
                "The provided default scheme is empty or all whitespace. Please provide a non-empty default scheme.",
            ));
        }

        if !allowed_schema.contains(&default_scheme) {
            return Err(Error::for_system(
                Kind::InvalidInput,
                format!(
                    "The provided default scheme '{}' is not in the list of allowed schemes: {:?}. \
                Either provide a default scheme that is in the list of allowed schemes, or update the list of \
                allowed schema to include the default schema.",
                    default_scheme, allowed_schema
                ),
            ));
        }
        Ok(default_scheme)
    }

    fn verify_repo_link(
        &mut self,
        allowed_schema: &Vec<String>,
        default_schema: &str,
    ) -> Result<String, Error> {
        let existing_link = self.repo_link.take().ok_or_else(|| {
            Error::for_system(
                Kind::InvalidInput,
                "A repository link was not provided. Please provide a repository link.",
            )
        })?;

        if existing_link.trim().is_empty() {
            return Err(Error::for_system(
                Kind::InvalidInput,
                "The provided repository link is empty or all whitespace. Please provide a non-empty repository link.",
            ));
        }

        self.verify_repo_link_not_malformed(&existing_link)?;

        let repo_link_with_scheme =
            self.apply_default_scheme_to_repo_link(&existing_link, &default_schema);

        self.verify_repo_link_scheme_is_allowed(
            &repo_link_with_scheme,
            &allowed_schema,
            &default_schema,
        )?;

        Ok(repo_link_with_scheme)
    }

    fn verify_repo_link_not_malformed(&self, repo_link: &str) -> Result<(), Error> {
        // Check if the link starts with an alphanumeric character (assume host without scheme)
        if repo_link
            .chars()
            .next()
            .map_or(false, |c| c.is_alphanumeric())
        {
            // Proceed to apply default scheme
            Ok(())
        } else {
            // Must contain a proper scheme separator '://'
            let parts: Vec<&str> = repo_link.split("://").collect();
            if parts.len() != 2 || parts[1].is_empty() {
                return Err(Error::for_system(
                    Kind::InvalidInput,
                    format!(
                        "The repository link [{}] is malformed. It must either start with contain a scheme separator '://' \
            or be formatted as [scheme]://[host] per for the RFC 3986 specification.",
                        repo_link
                    ),
                ));
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
        verified_repo_link: &String,
        allowed_schemes: &[String],
        default_scheme: &str,
    ) -> Result<(), Error> {
        let repo_link_schema = self.extract_repo_link_schema(&verified_repo_link);
        if allowed_schemes.contains(&repo_link_schema) {
            Ok(())
        } else {
            Err(Error::for_system(
                Kind::InvalidInput,
                format!(
                    "The url scheme '{}' is not allowed. Allowed schemes are {:?} \
                    and the default scheme is '{}'.",
                    repo_link_schema, allowed_schemes, default_scheme
                ),
            ))
        }
    }

    fn extract_repo_link_schema(&self, link: &str) -> String {
        let url_parts: Vec<&str> = link.split("://").collect();
        url_parts[0].to_string()
    }
}
