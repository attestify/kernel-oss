//! Tests for private URL-engine module boundaries.

#[test]
/// Verifies the private engine can consume its private IDNA seam.
fn idna_seam_is_available_inside_engine_success() {
    assert_eq!(
        super::idna::to_ascii("example", false),
        Ok("example".to_owned())
    );
}
