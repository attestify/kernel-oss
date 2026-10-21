//! Deliberately nonconforming fixture; the compiler harness owns its verification.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F30.

use kernel_oss::logging::Logger;

/// Sole local type for the `result_return` contract boundary; no runtime construction.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F30.
pub(crate) struct ResultReturn;

/// Intentional trait violation checked by error code and obligation text.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T08; Layout: F30.
impl Logger for ResultReturn {
    fn info(&self, _: &str) -> Result<(), kernel_oss::error::Error> {
        Ok(())
    }
    fn warn(&self, _: &str) {}
    fn error(&self, _: &str) {}
}
