//! Observes only ordinary Logger calls; no production private-state access.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T05; Layout: F16.
//! Verified through the consumer suite, not a duplicate helper-only suite.

use kernel_oss::logging::Logger;
use std::sync::Mutex;

/// Records the operation and exact borrowed text for consumer assertions.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T05; Layout: F16.
#[derive(Default)]
pub(crate) struct RecordingLogger {
    records: Mutex<Vec<(&'static str, String)>>,
}

impl RecordingLogger {
    /// Returns an owned observation without exposing mutable fixture state.
    pub(crate) fn snapshot(&self) -> Vec<(&'static str, String)> {
        self.records
            .lock()
            .expect("recording fixture must not be poisoned")
            .clone()
    }

    /// Serializes a test observation of one ordinary Logger operation.
    fn record(&self, operation: &'static str, message: &str) {
        self.records
            .lock()
            .expect("recording fixture must not be poisoned")
            .push((operation, message.to_owned()));
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T05; Layout: F16.
impl Logger for RecordingLogger {
    fn info(&self, message: &str) {
        self.record("info", message);
    }
    fn warn(&self, message: &str) {
        self.record("warn", message);
    }
    fn error(&self, message: &str) {
        self.record("error", message);
    }
}
