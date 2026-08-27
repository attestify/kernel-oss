//! Repository-link builder contract tests.

use super::RepositoryLink;
use crate::error::{Audience, Kind};

fn allowed_schemes() -> Vec<String> {
    ["file", "git", "https"]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn assert_repository_error(
    result: Result<RepositoryLink, crate::error::Error>,
    audience: Audience,
) {
    assert!(result.is_err());
    let error = match result {
        Ok(_) => panic!("repository link unexpectedly succeeded"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), Kind::InvalidInput);
    assert_eq!(error.audience(), audience);
    assert_eq!(error.message(), "Repository link validation failed.");
}

#[test]
/// Builds an already-qualified repository link through the new public URL facade.
fn build_success() {
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schemes())
        .default_scheme("git")
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();
    assert!(result.is_ok());
    let repository_link = match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected repository-link failure: {error}"),
    };
    assert_eq!(
        repository_link.url().value(),
        "https://github.com/nape/processes/rust-ci"
    );
    assert_eq!(repository_link.url().scheme(), "https");
    assert_eq!(repository_link.url().host(), Some("github.com"));
    assert_eq!(repository_link.url().port(), None);
    assert_eq!(repository_link.url().path(), "/nape/processes/rust-ci");
    assert_eq!(repository_link.url().query(), None);
    assert_eq!(repository_link.url().fragment(), None);
    assert_eq!(repository_link.to_string(), "RepositoryLink([REDACTED])");
    assert_eq!(format!("{repository_link:?}"), "RepositoryLink([REDACTED])");
}

#[test]
/// Applies the configured default scheme to a bare repository location.
fn apply_default_scheme_success() {
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schemes())
        .default_scheme("git")
        .repo_link("github.com/nape/processes/rust-ci")
        .build();
    assert!(result.is_ok());
    let repository_link = match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected repository-link failure: {error}"),
    };
    assert_eq!(
        repository_link.url().value(),
        "git://github.com/nape/processes/rust-ci"
    );
}

#[test]
/// Applies the configured default scheme to a separator-only input.
fn apply_default_scheme_with_scheme_seperator_only_success() {
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schemes())
        .default_scheme("git")
        .repo_link("://github.com/nape/processes/rust-ci")
        .build();
    assert!(result.is_ok());
    let repository_link = match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected repository-link failure: {error}"),
    };
    assert_eq!(
        repository_link.url().value(),
        "git://github.com/nape/processes/rust-ci"
    );
}

#[test]
/// Rejects a malformed repository link as a system-facing configuration failure.
fn malformed_repo_link_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("git")
            .repo_link(":/github.com/nape/processes/rust-ci")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects missing allowed schemes as a system-facing configuration failure.
fn no_allowed_schema_provided_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .default_scheme("git")
            .repo_link("https://github.com/nape/processes/rust-ci")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects a missing default scheme as a user-facing configuration failure.
fn no_default_scheme_provided_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .repo_link("https://github.com/nape/processes/rust-ci")
            .build(),
        Audience::User,
    );
}

#[test]
/// Distinguishes an explicitly supplied empty default scheme from an absent one.
fn empty_default_scheme_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("")
            .repo_link("https://github.com/nape/processes/rust-ci")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects a default scheme outside the configured allow-list.
fn invalid_default_scheme_not_in_allowed_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("ssh")
            .repo_link("https://github.com/nape/processes/rust-ci")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects a missing repository link after valid builder configuration.
fn no_repo_link_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("git")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects whitespace-only repository links without disclosing their content.
fn repo_link_whitespace_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("git")
            .repo_link("    ")
            .build(),
        Audience::System,
    );
}

#[test]
/// Rejects a repository link whose explicit scheme is not allowed.
fn unallowed_repo_link_scheme_error() {
    assert_repository_error(
        RepositoryLink::builder()
            .allowed_schema(allowed_schemes())
            .default_scheme("git")
            .repo_link("ssh://github.com/nape/processes/rust-ci")
            .build(),
        Audience::System,
    );
}

#[test]
/// Redacts all automatic repository-link formatting and error paths.
fn automatic_diagnostics_are_redacted_success() {
    let success = RepositoryLink::builder()
        .allowed_schema(allowed_schemes())
        .default_scheme("https")
        .repo_link("https://canaryuser:canarypass@example.test/?canaryquery#canaryfragment")
        .build();
    assert!(success.is_ok());
    let success = match success {
        Ok(value) => value,
        Err(error) => panic!("unexpected repository-link failure: {error}"),
    };
    assert!(success.url().value().contains("canaryuser"));
    assert_eq!(success.to_string(), "RepositoryLink([REDACTED])");
    assert_eq!(format!("{success:?}"), "RepositoryLink([REDACTED])");

    let failure = RepositoryLink::builder()
        .allowed_schema(allowed_schemes())
        .default_scheme("git")
        .repo_link(":/canaryuser:canarypass?canaryquery#canaryfragment")
        .build();
    assert!(failure.is_err());
    let failure = match failure {
        Ok(_) => panic!("malformed repository link unexpectedly succeeded"),
        Err(error) => error,
    };
    assert_eq!(failure.message(), "Repository link validation failed.");
    for canary in ["canaryuser", "canarypass", "canaryquery", "canaryfragment"] {
        assert!(!failure.message().contains(canary));
    }
}
