//! Shared gateway seam for writing log entries.
//!
//! Logging is modeled as one command capability: write a log entry. The log
//! level is request data, not a separate architectural seam. This keeps the
//! public API smaller than separate error, warning, info, and debug gateway
//! traits while preserving the ability to route on level inside an adapter.
//!
//! A log entry has a required message even when it also carries an [`Error`].
//! The message describes the event or operation being logged. The optional
//! error carries structured failure context for that event. This distinction
//! prevents callers from creating fake errors for normal info/debug logs and
//! avoids overloading `Error::message` as the entire log line for failure logs.

#[cfg(test)]
mod tests;

use crate::error::{Error, Kind};
use crate::gateway::{AsyncGateway, Gateway};
use crate::response::ResponseFuture;

/// Classifies the severity or purpose of a log entry.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LogLevel {
    /// A failure or exceptional condition.
    Error,
    /// A recoverable or noteworthy condition.
    Warning,
    /// A normal operational event.
    Info,
    /// Diagnostic detail for development or troubleshooting.
    Debug,
}

/// Built request for writing one log entry.
///
/// `message` is the primary event text. `error` is optional structured failure
/// context attached to the event. `context` is optional diagnostic detail such
/// as identifiers, external resource names, or caller-specific state.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WriteLogEntryRequest {
    level: LogLevel,
    message: String,
    error: Option<Error>,
    context: Option<String>,
}

impl WriteLogEntryRequest {
    /// Starts construction for a [`WriteLogEntryRequest`].
    pub fn builder() -> WriteLogEntryRequestBuilder {
        WriteLogEntryRequestBuilder::default()
    }

    /// Returns the log level for this entry.
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Returns the primary log event message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns optional structured error context for the log event.
    pub fn error(&self) -> Option<&Error> {
        self.error.as_ref()
    }

    /// Returns optional additional diagnostic context for the log event.
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

/// Builds a [`WriteLogEntryRequest`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WriteLogEntryRequestBuilder {
    level: Option<LogLevel>,
    message: Option<String>,
    error: Option<Error>,
    context: Option<String>,
}

impl WriteLogEntryRequestBuilder {
    /// Sets the log level for the entry.
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Sets the primary log event message.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Attaches structured failure context to the log entry.
    pub fn error(mut self, error: Error) -> Self {
        self.error = Some(error);
        self
    }

    /// Attaches additional diagnostic context to the log entry.
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Validates and returns a built [`WriteLogEntryRequest`].
    pub fn try_build(mut self) -> Result<WriteLogEntryRequest, Error> {
        let level = self.validate_level()?;
        let message = self.validate_message()?;
        let context = self.validate_context()?;

        Ok(WriteLogEntryRequest {
            level,
            message,
            error: self.error.take(),
            context,
        })
    }

    fn validate_level(&mut self) -> Result<LogLevel, Error> {
        self.level.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A write log entry level is required, but none was provided.",
            )
        })
    }

    fn validate_message(&mut self) -> Result<String, Error> {
        let message = self.message.take().ok_or_else(|| {
            Error::for_user(
                Kind::InvalidInput,
                "A write log entry message is required, but none was provided.",
            )
        })?;

        if message.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The write log entry message provided is empty, provide a non-empty write log entry message.",
            ));
        }

        if message.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The write log entry message provided contains only whitespace, provide a non-empty write log entry message.",
            ));
        }

        Ok(message)
    }

    fn validate_context(&mut self) -> Result<Option<String>, Error> {
        let Some(context) = self.context.take() else {
            return Ok(None);
        };

        if context.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The write log entry context provided is empty, omit context or provide non-empty write log entry context.",
            ));
        }

        if context.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The write log entry context provided contains only whitespace, omit context or provide non-empty write log entry context.",
            ));
        }

        Ok(Some(context))
    }
}

/// Defines the domain seam for writing one log entry.
pub trait WriteLogEntryGW: Gateway<Request = WriteLogEntryRequest, Response = ()> {}

/// Defines the asynchronous domain seam for writing one log entry.
pub trait AsyncWriteLogEntryGW:
    AsyncGateway<Request = WriteLogEntryRequest, Response = ()>
{
}

/// Adapts a log-entry function to the shared gateway seam.
#[derive(Clone, Copy)]
pub struct WriteLogEntryFnGateway {
    write_log_entry: fn(request: WriteLogEntryRequest) -> Result<(), Error>,
}

impl WriteLogEntryFnGateway {
    /// Creates a gateway adapter from a log-entry function.
    pub fn new(write_log_entry: fn(request: WriteLogEntryRequest) -> Result<(), Error>) -> Self {
        Self { write_log_entry }
    }
}

impl Gateway for WriteLogEntryFnGateway {
    type Request = WriteLogEntryRequest;
    type Response = ();

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        (self.write_log_entry)(request)
    }
}

impl WriteLogEntryGW for WriteLogEntryFnGateway {}

impl AsyncGateway for WriteLogEntryFnGateway {
    type Request = WriteLogEntryRequest;
    type Response = ();

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move { Gateway::execute(self, request) })
    }
}

impl AsyncWriteLogEntryGW for WriteLogEntryFnGateway {}
