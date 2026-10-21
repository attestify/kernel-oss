//! Verifies the actual no-op through its canonical production interface.
//!
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test
//! Targets: LOG-T01, LOG-T02, LOG-T03, LOG-T04, LOG-T06, LOG-T07
//! Layout: F05
//! Test size: SM
//!
//! Covers explicit construction, unit returns, text boundaries, borrow expiry
//! and repeated/concurrent sharing. These observations do not prove arbitrary
//! absence of copies or I/O; source review and subprocess checks complement them.
//! No concrete sink behavior or production delivery is under test.

use crate::logging::{Logger, NoOpLogger};
use std::sync::Arc;
use std::thread;

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T01
/// Layout: F05
/// Test size: SM
/// Verifies both ordinary constructors produce usable equivalent no-op interfaces.
#[test]
fn new_and_default_construct_usable_no_ops_success() {
    let constructed = NoOpLogger::new();
    let defaulted = NoOpLogger::default();
    for logger in [&constructed as &dyn Logger, &defaulted as &dyn Logger] {
        let (): () = logger.info("information");
        let (): () = logger.warn("warning");
        let (): () = logger.error("failure");
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T02
/// Layout: F05
/// Test size: SM
/// Verifies info returns unit normally for empty, whitespace and Unicode text.
#[test]
fn info_accepts_text_boundaries_success() {
    let logger = NoOpLogger::new();
    for message in ["", " \t\n", "é 日本語 🦀"] {
        let (): () = logger.info(message);
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T03
/// Layout: F05
/// Test size: SM
/// Verifies warn returns unit normally for empty, whitespace and Unicode text.
#[test]
fn warn_accepts_text_boundaries_success() {
    let logger = NoOpLogger::new();
    for message in ["", " \t\n", "é 日本語 🦀"] {
        let (): () = logger.warn(message);
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T04
/// Layout: F05
/// Test size: SM
/// Verifies error returns unit normally for empty, whitespace and Unicode text.
#[test]
fn error_accepts_text_boundaries_success() {
    let logger = NoOpLogger::new();
    for message in ["", " \t\n", "é 日本語 🦀"] {
        let (): () = logger.error(message);
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T07
/// Layout: F05
/// Test size: SM
/// Verifies message borrowing ends at return while the logger remains usable.
/// This does not by itself prove that no implementation copies the message.
#[test]
fn borrowed_source_can_change_and_expire_success() {
    let logger = NoOpLogger::new();
    let mut message = String::from("borrowed text");
    logger.info(&message);
    message.push_str(" changed");
    logger.warn(&message);
    message.clear();
    logger.error(&message);
    drop(message);
    let (): () = logger.info("logger still usable");
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T06
/// Layout: F05
/// Test size: SM
/// Verifies finite repeated ordinary calls return unit without initialization.
#[test]
fn repeated_calls_return_normally_success() {
    let logger = NoOpLogger::new();
    for _ in 0..32 {
        let (): () = logger.info("repeat");
        let (): () = logger.warn("repeat");
        let (): () = logger.error("repeat");
    }
}

/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test
/// Targets: LOG-T06
/// Layout: F05
/// Test size: SM
/// Verifies shared no-op calls complete normally on concurrent threads.
/// Source inspection separately establishes absence of retained invocation state.
#[test]
fn concurrent_shared_calls_return_normally_success() {
    let logger: Arc<dyn Logger> = Arc::new(NoOpLogger::new());
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let logger = Arc::clone(&logger);
            thread::spawn(move || {
                for _ in 0..8 {
                    let (): () = logger.info("shared information");
                    let (): () = logger.warn("shared warning");
                    let (): () = logger.error("shared failure");
                }
            })
        })
        .collect();
    for worker in threads {
        worker.join().expect("no-op worker must finish normally");
    }
}
