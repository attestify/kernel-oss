use crate::error::{Audience, Kind};
use crate::values::specification::repository_link::RepositoryLink;
use std::string::ToString;
use test_framework_oss::{is_error, is_ok, kernel_error_eq};

#[test]
fn build_success() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let default_schema = "git";

    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();

    let repository_link = is_ok!(result);

    assert_eq!(
        repository_link.to_string(),
        "https://github.com/nape/processes/rust-ci"
    );
    assert_eq!(repository_link.url().scheme, "https");
    assert_eq!(repository_link.url().host, "github.com");
    assert_eq!(repository_link.url().port, 0);
    assert_eq!(repository_link.url().path, "/nape/processes/rust-ci");
    assert_eq!(repository_link.url().query_string, "");
    assert_eq!(repository_link.url().queries.len(), 0);
    assert_eq!(repository_link.url().fragment, "");
}

#[test]
fn apply_default_scheme_success() {
    let allowed_schema = ["file".to_string(), "git".to_string()].to_vec();
    let default_schema = "git";

    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link("github.com/nape/processes/rust-ci")
        .build();

    let repository_link = is_ok!(result);

    assert_eq!(
        repository_link.to_string(),
        "git://github.com/nape/processes/rust-ci"
    );
    assert_eq!(repository_link.url().scheme, "git");
    assert_eq!(repository_link.url().host, "github.com");
    assert_eq!(repository_link.url().port, 0);
    assert_eq!(repository_link.url().path, "/nape/processes/rust-ci");
    assert_eq!(repository_link.url().query_string, "");
    assert_eq!(repository_link.url().queries.len(), 0);
    assert_eq!(repository_link.url().fragment, "");
}

#[test]
fn apply_default_scheme_with_scheme_seperator_only_success() {
    let allowed_schema = ["file".to_string(), "git".to_string()].to_vec();
    let default_schema = "git";

    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link("://github.com/nape/processes/rust-ci")
        .build();

    let repository_link = is_ok!(result);

    assert_eq!(
        repository_link.to_string(),
        "git://github.com/nape/processes/rust-ci"
    );
    assert_eq!(repository_link.url().scheme, "git");
    assert_eq!(repository_link.url().host, "github.com");
    assert_eq!(repository_link.url().port, 0);
    assert_eq!(repository_link.url().path, "/nape/processes/rust-ci");
    assert_eq!(repository_link.url().query_string, "");
    assert_eq!(repository_link.url().queries.len(), 0);
    assert_eq!(repository_link.url().fragment, "");
}

#[test]
fn malformed_repo_link_error() {
    let allowed_schema = ["file".to_string(), "git".to_string()].to_vec();
    let default_schema = "git";

    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link(":/github.com/nape/processes/rust-ci")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "The repository link [:/github.com/nape/processes/rust-ci] is malformed. \
        It must either start with contain a scheme separator '://' or be formatted as \
        [scheme]://[host] per for the RFC 3986 specification."
    )
}

#[test]
fn no_allowed_schema_provided_error() {
    let default_schema = "git";
    let result = RepositoryLink::builder()
        .default_scheme(default_schema)
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "No allowed schemes were provided, please provide at least one allowed scheme."
    )
}

#[test]
fn no_default_scheme_provided_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::User,
        "A default scheme was not provided. Please provide a default scheme."
    )
}

#[test]
fn empty_default_scheme_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme("")
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "The provided default scheme is empty or all whitespace. Please provide a non-empty default scheme."
    )
}

#[test]
fn invalid_default_scheme_not_in_allowed_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme("ssh")
        .repo_link("https://github.com/nape/processes/rust-ci")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "The provided default scheme 'ssh' is not in the list of allowed schemes: [\"file\", \"git\", \"https\"]. \
        Either provide a default scheme that is in the list of allowed schemes, or update the list of \
        allowed schema to include the default schema."
    )
}

#[test]
fn no_repo_link_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let default_schema = "git";
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "A repository link was not provided. Please provide a repository link."
    )
}

#[test]
fn repo_link_whitespace_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let default_schema = "git";
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link("    ")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "The provided repository link is empty or all whitespace. Please provide a non-empty repository link."
    )
}

#[test]
fn unallowed_repo_link_scheme_error() {
    let allowed_schema = ["file".to_string(), "git".to_string(), "https".to_string()].to_vec();
    let default_schema = "git";
    let result = RepositoryLink::builder()
        .allowed_schema(allowed_schema)
        .default_scheme(default_schema)
        .repo_link("ssh://localhost")
        .build();

    is_error!(&result);

    kernel_error_eq!(
        &result,
        Kind::InvalidInput,
        Audience::System,
        "The url scheme 'ssh' is not allowed. Allowed schemes are [\"file\", \"git\", \"https\"] and the default scheme is 'git'."
    )
}
