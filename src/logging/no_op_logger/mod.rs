//! Explicit stateless implementation of the operational Logger.
//!
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: implementation
//! Targets: LOG-T01, LOG-T02, LOG-T03, LOG-T04, LOG-T06, LOG-T07
//! Layout: F04

use crate::logging::Logger;

#[cfg(test)]
mod tests;

/// A logger that deliberately discards all messages without effects.
///
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: implementation
/// Targets: LOG-T01, LOG-T02, LOG-T03, LOG-T04, LOG-T06, LOG-T07
/// Layout: F04
///
/// Construction is infallible and has no configuration. Calls accept any text,
/// perform no allocation or I/O, retain no input or invocation state, and
/// return normally with unit. Repeated and concurrent calls behave identically.
/// This is deliberate absence of logging, not degraded output, so no failure
/// diagnostics are emitted. `Default` does not install an implicit dependency.
pub struct NoOpLogger {
    _private: (),
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: implementation
/// Targets: LOG-T01
/// Layout: F04
impl NoOpLogger {
    /// Constructs an explicit no-op without configuration, resources or failure.
    pub fn new() -> Self {
        Self { _private: () }
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: implementation
/// Targets: LOG-T01
/// Layout: F04
impl Default for NoOpLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: implementation
/// Targets: LOG-T02, LOG-T03, LOG-T04, LOG-T06, LOG-T07
/// Layout: F04
impl Logger for NoOpLogger {
    /// Ignores the message, performs no delivery and returns unit normally.
    fn info(&self, _message: &str) {}

    /// Ignores the message, performs no delivery and returns unit normally.
    fn warn(&self, _message: &str) {}

    /// Ignores the message, performs no delivery and returns unit normally.
    fn error(&self, _message: &str) {}
}
