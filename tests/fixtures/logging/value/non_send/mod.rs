//! Deliberately nonconforming fixture; the compiler harness owns its verification.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F27.

use kernel_oss::logging::Logger;

/// Sole local type for the `non_send` contract boundary; no runtime construction.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F27.
pub(crate) struct NonSend {
    _guard: std::sync::MutexGuard<'static, ()>,
}

/// Intentional trait violation checked by error code and obligation text.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F27.
impl Logger for NonSend {
    fn info(&self, _: &str) {}
    fn warn(&self, _: &str) {}
    fn error(&self, _: &str) {}
}
