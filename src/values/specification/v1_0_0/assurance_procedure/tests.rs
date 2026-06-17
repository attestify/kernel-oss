//! Tests for the v1.0.0 assurance-procedure wrapper, covering builder success and validation.
//!
//! Bounded unit under test: the `v1_0_0::AssuranceProcedure` builder wrapper.
//! Public interfaces verified: `builder`, `AssuranceProcedureBuilder::default`, and `try_build`.
//! Logical paths covered: valid construction, missing API version, invalid API version, missing
//! procedure data, and duplicate artifact rejection.
//! Requirement validation points: standards-aligned compatibility wrapper behavior for
//! assurance-procedure construction.

use crate::error;
use crate::values::specification::api_version::APIVersion;
use crate::values::specification::assurance_procedure::activity::Activity;
use crate::values::specification::assurance_procedure::artifact::Artifact;
use crate::values::specification::kind::Kind;
use crate::values::specification::v1_0_0::assurance_procedure::{
    AssuranceProcedure, AssuranceProcedureBuilder,
};
use test_framework_oss::{is_error, is_ok};

fn procedure_activity() -> Activity {
    is_ok!(Activity::new("procedure-1", "Short Desc", "Long Desc"))
}

fn procedure_artifact() -> Artifact {
    is_ok!(Artifact::new(
        "artifact-1",
        "Short Desc",
        &[("key".to_string(), "value".to_string())],
    ))
}

fn builder() -> AssuranceProcedureBuilder {
    AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info(
            "nrn:sourcecode::example",
            "A Short Desc.",
            "This is an example procedure",
        )
        .add_activity(&procedure_activity())
}

#[test]
/// Requirement validation: verifies the builder creates a valid assurance procedure.
fn new_builder() {
    let result = builder().try_build();
    let result = is_ok!(result);

    assert_eq!(result.api_version, is_ok!("1.0.0".parse::<APIVersion>()));
    assert_eq!(result.kind, Kind::AssuranceProcedure);
}

#[test]
/// Requirement validation: verifies the default builder creates a valid assurance procedure.
fn default_builder_success() {
    let result = AssuranceProcedureBuilder::default()
        .api_version("1.0.0")
        .procedure_info(
            "nrn:sourcecode::example",
            "A Short Desc.",
            "This is an example procedure",
        )
        .add_activity(&procedure_activity())
        .try_build();
    let result = is_ok!(result);

    assert_eq!(result.api_version, is_ok!("1.0.0".parse::<APIVersion>()));
    assert_eq!(result.kind, Kind::AssuranceProcedure);
}

#[test]
/// Requirement validation: verifies the builder rejects a missing API version.
fn builder_error_missing_api_version() {
    let result = AssuranceProcedure::builder()
        .procedure_info(
            "nrn:sourcecode::example",
            "A Short Desc.",
            "This is an example procedure",
        )
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with("The AssuranceProcedure could not be created: The APIVersion is required, but was not provided."));
}

#[test]
/// Requirement validation: verifies the builder rejects an invalid API version.
fn builder_error_invalid_api_version() {
    let result = AssuranceProcedure::builder()
        .api_version("")
        .procedure_info(
            "nrn:sourcecode::example",
            "A Short Desc.",
            "This is an example procedure",
        )
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(
        err.message.starts_with(
            "The AssuranceProcedure could not be created: The APIVersion has an issue: "
        )
    );
}

#[test]
/// Requirement validation: verifies the builder rejects missing procedure information.
fn builder_error_missing_nrn() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert_eq!(
        "The AssuranceProcedure could not be created: Check that you provided the procedure info. Either the procedure NRN, short description, or long description is missing.",
        err.message
    );
}

#[test]
/// Requirement validation: verifies the builder rejects an invalid procedure NRN.
fn builder_error_invalid_procedure_nrn() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("", "A Short Desc.", "This is an example procedure")
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with(
        "The AssuranceProcedure could not be created: The procedure information has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies the builder rejects an invalid procedure short description.
fn builder_error_invalid_procedure_short() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info(
            "nrn:sourcecode::example",
            "",
            "This is an example procedure",
        )
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with(
        "The AssuranceProcedure could not be created: The procedure information has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies the builder rejects an invalid procedure description.
fn builder_error_invalid_procedure_description() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info("nrn:sourcecode::example", "A Short Desc.", "")
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with(
        "The AssuranceProcedure could not be created: The procedure information has an issue: "
    ));
}

#[test]
/// Requirement validation: verifies duplicate artifacts are rejected during build.
fn builder_error_add_artifact_duplicate_artifact() {
    let result = AssuranceProcedure::builder()
        .api_version("1.0.0")
        .procedure_info(
            "nrn:sourcecode::example",
            "A Short Desc.",
            "This is an example procedure",
        )
        .add_activity(&procedure_activity())
        .add_artifact(&procedure_artifact())
        .add_artifact(&procedure_artifact())
        .try_build();

    let err = is_error!(result);
    assert_eq!(err.kind, error::Kind::InvalidInput);
    assert_eq!(err.audience, error::Audience::User);
    assert!(err.message.starts_with(
        "The AssuranceProcedure could not be created: The artifact 'artifact-1' has an issue: "
    ));
}
