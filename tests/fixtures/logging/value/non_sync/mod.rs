//! Deliberately nonconforming fixture; the compiler harness owns its verification.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F28.

use kernel_oss::logging::Logger;

/// Sole local type for the `non_sync` contract boundary; no runtime construction.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F28.
pub(crate) struct NonSync {
    _cell: std::cell::Cell<()>,
}

/// Intentional trait violation checked by error code and obligation text.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F28.
impl Logger for NonSync {
    fn info(&self, _: &str) {}
    fn warn(&self, _: &str) {}
    fn error(&self, _: &str) {}
}
