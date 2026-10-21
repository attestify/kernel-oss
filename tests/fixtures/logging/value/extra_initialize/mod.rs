//! Deliberately nonconforming fixture; the compiler harness owns its verification.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F29.

use kernel_oss::logging::Logger;

/// Sole local type for the `extra_initialize` contract boundary; no runtime construction.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F29.
pub(crate) struct ExtraInitialize;

/// Intentional trait violation checked by error code and obligation text.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F29.
impl Logger for ExtraInitialize {
    fn info(&self, _: &str) {}
    fn warn(&self, _: &str) {}
    fn error(&self, _: &str) {}
    fn initialize(&self) {}
}
