use crate::error::Error;

pub trait Logger: Sync + Send {
    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to log.
    /// * `additional_context` - Additional context to log that can providing context to the error.
    ///
    fn error(&self, error: Error, additional_context: Option<&str>);

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `warning` - The warning message to log.
    /// * `error` - An optional error that can provide context to the warning.
    ///
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
    fn info(&self, info: &str, error: Option<Error>);

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `debug` - The debug message to log.
    ///
    fn debug(&self, debug: &str);
}
