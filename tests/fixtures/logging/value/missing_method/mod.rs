//! Deliberately nonconforming fixture; the compiler harness owns its verification.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F26.

use kernel_oss::logging::Logger;

/// Sole local type for the `missing_method` contract boundary; no runtime construction.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F26.
pub(crate) struct MissingMethod;

/// Intentional trait violation checked by error code and obligation text.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F26.
impl Logger for MissingMethod {
    fn info(&self, _: &str) {}
    fn error(&self, _: &str) {}
}
