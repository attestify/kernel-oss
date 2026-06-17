//! Verifies the legacy logger compatibility surface.
//!
//! Bounded unit under test:
//! - `Logger`
//!
//! Public interfaces verified:
//! - `Logger::error`
//! - `Logger::warning`
//! - `Logger::info`
//! - `Logger::debug`
//!
//! Logical paths covered:
//! - each legacy log level forwards a message and optional error context
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

#![allow(deprecated)]

use crate::error::{Error, Kind};
use crate::gateway::logger::Logger;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct RecordingLogger {
    entries: Arc<Mutex<Vec<String>>>,
}

impl RecordingLogger {
    fn recorded_entries(&self) -> Vec<String> {
        match self.entries.lock() {
            Ok(entries) => entries.clone(),
            Err(_) => panic!("expected logger entries lock"),
        }
    }
}

impl Logger for RecordingLogger {
    fn error(&self, error: Error, additional_context: Option<&str>) {
        match self.entries.lock() {
            Ok(mut entries) => entries.push(format!(
                "error:{}:{}",
                error.message,
                additional_context.unwrap_or("")
            )),
            Err(_) => panic!("expected logger entries lock"),
        }
    }

    fn warning(&self, warning: &str, error: Option<Error>) {
        match self.entries.lock() {
            Ok(mut entries) => entries.push(format!(
                "warning:{}:{}",
                warning,
                error.map(|err| err.message).unwrap_or_default()
            )),
            Err(_) => panic!("expected logger entries lock"),
        }
    }

    fn info(&self, info: &str, error: Option<Error>) {
        match self.entries.lock() {
            Ok(mut entries) => entries.push(format!(
                "info:{}:{}",
                info,
                error.map(|err| err.message).unwrap_or_default()
            )),
            Err(_) => panic!("expected logger entries lock"),
        }
    }

    fn debug(&self, debug: &str) {
        match self.entries.lock() {
            Ok(mut entries) => entries.push(format!("debug:{}", debug)),
            Err(_) => panic!("expected logger entries lock"),
        }
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that each legacy logging method forwards its data to the
/// implementation.
#[test]
fn logger_methods_success() {
    let logger = RecordingLogger::default();
    let error = Error::for_user(Kind::InvalidInput, "boom");

    logger.error(error.clone(), Some("context"));
    logger.warning("warn", Some(error.clone()));
    logger.info("info", None);
    logger.debug("debug");

    assert_eq!(
        logger.recorded_entries(),
        vec![
            "error:boom:context".to_string(),
            "warning:warn:boom".to_string(),
            "info:info:".to_string(),
            "debug:debug".to_string(),
        ]
    );
}
