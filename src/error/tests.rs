//! Direct logical-path evidence for bounded Error provenance.
//!
//! These tests exercise the public value and Error reconstruction API from its
//! owning module. The separate `tests/error_provenance_public_api.rs` test
//! compiles the same surface as an external consumer.

use super::{
    Audience, Error, ErrorProvenance, ErrorProvenanceCode, ErrorProvenanceDescriptor,
    ErrorProvenanceNamespace, ErrorProvenanceSchemaVersion, Kind,
};

const INVALID_NAMESPACE_MESSAGE: &str = "Error provenance namespace is invalid.";
const INVALID_CODE_MESSAGE: &str = "Error provenance code is invalid.";
const INVALID_SCHEMA_VERSION_MESSAGE: &str = "Error provenance schema version must be nonzero.";
const ATTACH_PRESENT_MESSAGE: &str =
    "Error provenance cannot be attached because it is already present.";
const DERIVE_ABSENT_MESSAGE: &str = "Error provenance cannot be derived because it is absent.";
const DERIVE_ORIGIN_MESSAGE: &str =
    "Error provenance cannot be derived because an origin is already present.";

fn success<T>(result: Result<T, Error>, fixture: &str) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("{fixture} must succeed: {error:?}"),
    }
}

fn provenance<'error>(error: &'error Error, fixture: &str) -> &'error ErrorProvenance {
    match error.provenance() {
        Some(value) => value,
        None => panic!("{fixture} must retain provenance"),
    }
}

fn assert_exact_error<T>(result: Result<T, Error>, kind: Kind, message: &str) {
    let error = match result {
        Ok(_) => panic!("the invalid logical path must return a bounded error"),
        Err(error) => error,
    };
    assert_eq!(error.audience(), Audience::System);
    assert_eq!(error.kind(), kind);
    assert_eq!(error.message(), message);
    assert!(error.provenance().is_none());
}

fn descriptor(code: &str) -> ErrorProvenanceDescriptor {
    ErrorProvenanceDescriptor::new(
        success(
            ErrorProvenanceNamespace::try_new("attestify.kernel"),
            "the provenance namespace fixture",
        ),
        success(
            ErrorProvenanceSchemaVersion::try_new(1),
            "the provenance schema-version fixture",
        ),
        success(
            ErrorProvenanceCode::try_new(code),
            "the provenance code fixture",
        ),
    )
}

fn outer_error() -> Error {
    Error::new(Audience::User, Kind::InvalidInput, "The value is invalid.")
}

/// Requirement validation: S1-T01.
///
/// Logical path: a lower-bound namespace retains every permitted segment
/// character class without normalization.
/// Observable result: construction succeeds and the accessor preserves text.
#[test]
fn provenance_namespace_lower_bound_and_permitted_characters_success() {
    let lower_bound = success(
        ErrorProvenanceNamespace::try_new("a.b"),
        "the lower-bound namespace fixture",
    );
    let permitted_characters = success(
        ErrorProvenanceNamespace::try_new("a.b-c1"),
        "the permitted-character namespace fixture",
    );
    assert_eq!(lower_bound.as_str(), "a.b");
    assert_eq!(permitted_characters.as_str(), "a.b-c1");
}

/// Requirement validation: S1-T02.
///
/// Logical path: an eight-segment, 128-byte namespace retains each permitted
/// segment form without rewriting.
/// Observable result: construction succeeds and the accessor preserves text.
#[test]
fn provenance_namespace_upper_bound_eight_segments_success() {
    let segments = [
        "a123456789012345",
        "b123456789012345",
        "c123456789012345",
        "d123456789012345",
        "e123456789012345",
        "f123456789012345",
        "g123456789012345",
        "h12345678",
    ];
    let value = segments.join(".");
    assert_eq!(value.len(), 128);
    let namespace = success(
        ErrorProvenanceNamespace::try_new(&value),
        "the upper-bound namespace fixture",
    );
    assert_eq!(namespace.as_str(), value);
}

/// Requirement validation: S1-T03.
///
/// Logical path: namespace input below the byte lower bound is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_too_short_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("a."),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: namespace input above the byte upper bound is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_too_long_error() {
    let value = [
        "a123456789012345",
        "b123456789012345",
        "c123456789012345",
        "d123456789012345",
        "e123456789012345",
        "f123456789012345",
        "g123456789012345",
        "h123456789",
    ]
    .join(".");
    assert_eq!(value.len(), 129);
    assert_exact_error(
        ErrorProvenanceNamespace::try_new(value),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: a namespace with fewer than two segments is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_too_few_segments_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("abc"),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: a namespace with more than eight segments is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_too_many_segments_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("a.b.c.d.e.f.g.h.i"),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: a segment beginning with a non-lowercase letter is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_invalid_first_character_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("a.1b"),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: an empty dot-separated namespace segment is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_empty_segment_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("a..b"),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T03.
///
/// Logical path: a namespace disallowed character is rejected.
/// Observable result: exact system invalid-input namespace error.
#[test]
fn provenance_namespace_disallowed_character_error() {
    assert_exact_error(
        ErrorProvenanceNamespace::try_new("a.b_c"),
        Kind::InvalidInput,
        INVALID_NAMESPACE_MESSAGE,
    );
}

/// Requirement validation: S1-T04.
///
/// Logical path: lower and upper code bounds retain each permitted character
/// class without normalization.
/// Observable result: construction succeeds and accessors preserve text.
#[test]
fn provenance_code_lower_and_upper_bounds_success() {
    let minimum = success(
        ErrorProvenanceCode::try_new("a"),
        "the minimum code fixture",
    );
    let maximum = format!("a{}", "b1_".repeat(21));
    assert_eq!(maximum.len(), 64);
    let upper = success(
        ErrorProvenanceCode::try_new(&maximum),
        "the maximum code fixture",
    );
    assert_eq!(minimum.as_str(), "a");
    assert_eq!(upper.as_str(), maximum);
}

/// Requirement validation: S1-T05.
///
/// Logical path: a code outside its byte bounds is rejected.
/// Observable result: exact system invalid-input code error.
#[test]
fn provenance_code_length_violation_error() {
    assert_exact_error(
        ErrorProvenanceCode::try_new("a".repeat(65)),
        Kind::InvalidInput,
        INVALID_CODE_MESSAGE,
    );
}

/// Requirement validation: S1-T05.
///
/// Logical path: an empty code is below the byte lower bound.
/// Observable result: exact system invalid-input code error.
#[test]
fn provenance_code_empty_error() {
    assert_exact_error(
        ErrorProvenanceCode::try_new(""),
        Kind::InvalidInput,
        INVALID_CODE_MESSAGE,
    );
}

/// Requirement validation: S1-T05.
///
/// Logical path: a code beginning with a non-lowercase letter is rejected.
/// Observable result: exact system invalid-input code error.
#[test]
fn provenance_code_invalid_first_character_error() {
    assert_exact_error(
        ErrorProvenanceCode::try_new("1code"),
        Kind::InvalidInput,
        INVALID_CODE_MESSAGE,
    );
}

/// Requirement validation: S1-T05.
///
/// Logical path: a code containing a disallowed character is rejected.
/// Observable result: exact system invalid-input code error.
#[test]
fn provenance_code_disallowed_character_error() {
    assert_exact_error(
        ErrorProvenanceCode::try_new("code-value"),
        Kind::InvalidInput,
        INVALID_CODE_MESSAGE,
    );
}

/// Requirement validation: S1-T06.
///
/// Logical path: the lowest and highest nonzero schema versions are accepted.
/// Observable result: each accessor returns its exact integer.
#[test]
fn provenance_schema_version_bounds_success() {
    let minimum = success(
        ErrorProvenanceSchemaVersion::try_new(1),
        "the minimum schema-version fixture",
    );
    let maximum = success(
        ErrorProvenanceSchemaVersion::try_new(u16::MAX),
        "the maximum schema-version fixture",
    );
    assert_eq!(minimum.get(), 1);
    assert_eq!(maximum.get(), u16::MAX);
}

/// Requirement validation: S1-T07.
///
/// Logical path: schema version zero is rejected.
/// Observable result: exact system invalid-input schema-version error.
#[test]
fn provenance_schema_version_zero_error() {
    assert_exact_error(
        ErrorProvenanceSchemaVersion::try_new(0),
        Kind::InvalidInput,
        INVALID_SCHEMA_VERSION_MESSAGE,
    );
}

/// Requirement validation: S1-T08.
///
/// Logical path: a descriptor receives valid bounded inputs.
/// Observable result: infallible construction retains exact immutable values.
#[test]
fn provenance_descriptor_infallible_construction_success() {
    let descriptor = descriptor("dependency_failed");
    assert_eq!(descriptor.namespace().as_str(), "attestify.kernel");
    assert_eq!(descriptor.schema_version().get(), 1);
    assert_eq!(descriptor.code().as_str(), "dependency_failed");
}

/// Requirement validation: S1-T09.
///
/// Logical path: each Error constructor creates its existing outer state.
/// Observable result: outer accessors and display are unchanged; provenance is absent.
#[test]
fn error_constructors_start_without_provenance_success() {
    let generic = Error::new(Audience::User, Kind::InvalidInput, "The value is invalid.");
    let user = Error::for_user(Kind::NotFound, "The record is missing.");
    let system = Error::for_system(Kind::GatewayError, "The dependency is unavailable.");
    assert_eq!(generic.audience(), Audience::User);
    assert_eq!(generic.kind(), Kind::InvalidInput);
    assert_eq!(generic.message(), "The value is invalid.");
    assert_eq!(generic.audience, Audience::User);
    assert_eq!(generic.kind, Kind::InvalidInput);
    assert_eq!(generic.message, "The value is invalid.");
    assert_eq!(format!("{generic}"), "The value is invalid.");
    assert!(generic.provenance().is_none());
    assert!(user.is_user());
    assert!(user.provenance().is_none());
    assert!(system.is_system());
    assert!(system.provenance().is_none());
}

/// Requirement validation: S1-T10.
///
/// Logical path: provenance is attached to an Error with no existing provenance.
/// Observable result: current is supplied, origin is absent, and outer state remains unchanged.
#[test]
fn error_attach_from_absent_provenance_success() {
    let original = outer_error();
    let current = descriptor("gateway_failed");
    let attached = success(
        original.try_with_attached_provenance(current.clone()),
        "attaching provenance to an absent Error",
    );
    let attached_provenance = provenance(&attached, "the attached Error");
    assert_eq!(attached.audience(), original.audience());
    assert_eq!(attached.kind(), original.kind());
    assert_eq!(attached.message(), original.message());
    assert_eq!(attached_provenance.current(), &current);
    assert!(attached_provenance.origin().is_none());
    assert!(original.provenance().is_none());
}

/// Requirement validation: S1-T11.
///
/// Logical path: provenance is attached when one already exists.
/// Observable result: exact transition error and unchanged caller-held Error.
#[test]
fn error_attach_when_provenance_present_error() {
    let original = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the attached Error fixture",
    );
    let preserved = original.clone();
    assert_exact_error(
        original.try_with_attached_provenance(descriptor("application_failed")),
        Kind::ProcessingFailure,
        ATTACH_PRESENT_MESSAGE,
    );
    assert_eq!(original, preserved);
}

/// Requirement validation: S1-T12.
///
/// Logical path: provenance derivation is requested while provenance is absent.
/// Observable result: exact transition error and unchanged caller-held Error.
#[test]
fn error_derive_when_provenance_absent_error() {
    let original = outer_error();
    let preserved = original.clone();
    assert_exact_error(
        original.try_with_derived_provenance(descriptor("application_failed")),
        Kind::ProcessingFailure,
        DERIVE_ABSENT_MESSAGE,
    );
    assert_eq!(original, preserved);
}

/// Requirement validation: S1-T13.
///
/// Logical path: provenance is derived from a current-only Error.
/// Observable result: supplied descriptor becomes current, prior current becomes origin, and outer state remains unchanged.
#[test]
fn error_derive_from_current_only_provenance_success() {
    let original = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the attached Error fixture",
    );
    let derived = descriptor("application_failed");
    let result = success(
        original.try_with_derived_provenance(derived.clone()),
        "deriving from current-only provenance",
    );
    let result_provenance = provenance(&result, "the derived Error");
    let original_provenance = provenance(&original, "the original Error");
    assert_eq!(result.audience(), original.audience());
    assert_eq!(result.kind(), original.kind());
    assert_eq!(result.message(), original.message());
    assert_eq!(result_provenance.current(), &derived);
    assert_eq!(
        result_provenance.origin(),
        Some(original_provenance.current())
    );
    assert!(original_provenance.origin().is_none());
}

/// Requirement validation: S1-T14.
///
/// Logical path: provenance derivation is requested when an origin already exists.
/// Observable result: exact transition error and unchanged current/origin/outer state.
#[test]
fn error_derive_when_origin_present_error() {
    let attached = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the attached Error fixture",
    );
    let original = success(
        attached.try_with_derived_provenance(descriptor("application_failed")),
        "the derived Error fixture",
    );
    let preserved = original.clone();
    assert_exact_error(
        original.try_with_derived_provenance(descriptor("transport_failed")),
        Kind::ProcessingFailure,
        DERIVE_ORIGIN_MESSAGE,
    );
    assert_eq!(original, preserved);
}

/// Requirement validation: S1-T15.
///
/// Logical path: provenance is cleared from an Error with no provenance.
/// Observable result: outer state is retained and provenance remains absent.
#[test]
fn error_clear_from_absent_provenance_success() {
    let original = outer_error();
    let cleared = original.without_provenance();
    assert_eq!(cleared, original);
    assert!(cleared.provenance().is_none());
}

/// Requirement validation: S1-T15.
///
/// Logical path: provenance is cleared from a current-only Error.
/// Observable result: every outer field is retained and provenance is absent.
#[test]
fn error_clear_from_current_only_provenance_success() {
    let original = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the attached Error fixture",
    );
    let cleared = original.without_provenance();
    assert_eq!(cleared.audience(), original.audience());
    assert_eq!(cleared.kind(), original.kind());
    assert_eq!(cleared.message(), original.message());
    assert!(cleared.provenance().is_none());
}

/// Requirement validation: S1-T15.
///
/// Logical path: provenance is cleared from an Error with current and origin.
/// Observable result: every outer field is retained and provenance is absent.
#[test]
fn error_clear_from_current_plus_origin_provenance_success() {
    let attached = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the attached Error fixture",
    );
    let original = success(
        attached.try_with_derived_provenance(descriptor("application_failed")),
        "the derived Error fixture",
    );
    let cleared = original.without_provenance();
    assert_eq!(cleared.audience(), original.audience());
    assert_eq!(cleared.kind(), original.kind());
    assert_eq!(cleared.message(), original.message());
    assert!(cleared.provenance().is_none());
}

/// Requirement validation: S1-T16.
///
/// Logical path: two Errors have identical outer and provenance state.
/// Observable result: complete Error equality returns true.
#[test]
fn error_full_state_equality_success() {
    let left = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the left Error fixture",
    );
    let right = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the right Error fixture",
    );
    assert_eq!(left, right);
}

/// Requirement validation: S1-T17.
///
/// Logical path: Errors have equal outer fields but different current provenance.
/// Observable result: complete Error equality returns false.
#[test]
fn error_distinct_current_provenance_inequality_success() {
    let left = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the left Error fixture",
    );
    let right = success(
        outer_error().try_with_attached_provenance(descriptor("application_failed")),
        "the right Error fixture",
    );
    assert_ne!(left, right);
}

/// Requirement validation: S1-T17.
///
/// Logical path: Errors have equal outer and current provenance but distinct origins.
/// Observable result: complete Error equality returns false.
#[test]
fn error_distinct_origin_provenance_inequality_success() {
    let left_attached = success(
        outer_error().try_with_attached_provenance(descriptor("gateway_failed")),
        "the left attached Error fixture",
    );
    let right_attached = success(
        outer_error().try_with_attached_provenance(descriptor("transport_failed")),
        "the right attached Error fixture",
    );
    let left = success(
        left_attached.try_with_derived_provenance(descriptor("application_failed")),
        "the left derived Error fixture",
    );
    let right = success(
        right_attached.try_with_derived_provenance(descriptor("application_failed")),
        "the right derived Error fixture",
    );
    assert_ne!(left, right);
}
