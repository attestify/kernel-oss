//! Legacy logger compatibility surface.
//!
//! Prefer [`crate::gateway::write_log_entry::WriteLogEntryGW`] and
//! [`crate::gateway::write_log_entry::AsyncWriteLogEntryGW`] in new code.

#[cfg(test)]
mod tests;

use crate::error::Error;

#[deprecated(
    note = "Use gateway::write_log_entry::WriteLogEntryGW, which implements the shared Gateway seam."
)]
/// A legacy logging gateway trait kept for compatibility.
pub trait Logger: Sync + Send {
    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to log.
    /// * `additional_context` - Additional context to log that can providing context to the error.
    ///
    /// Emits an error-level log entry.
    fn error(&self, error: Error, additional_context: Option<&str>);

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `warning` - The warning message to log.
    /// * `error` - An optional error that can provide context to the warning.
    ///
    /// Emits a warning-level log entry.
    fn warning(&self, warning: &str, error: Option<Error>);

    /// Logs an information message.
    ///
    /// # Arguments
    ///
    /// * `info` - The information message to log.
    /// * `error` - An optional error that can provide context to the information message.
    ///
    /// When an [Error] is provided with the information message, the error is logged as the context to the information message.
    ///
    /// Emits an info-level log entry.
    fn info(&self, info: &str, error: Option<Error>);

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `debug` - The debug message to log.
    ///
    /// Emits a debug-level log entry.
    fn debug(&self, debug: &str);
}
