//! Verifies canonical external Logger imports and dynamic dispatch.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T01, LOG-T05, LOG-T09; Layout: F06.
//! Test size: MD. Paths: distinct borrowed calls, concurrent Arc sharing,
//! and explicit no-op construction. No production sink behavior is claimed.

mod logging_support;

use kernel_oss::logging::{Logger, NoOpLogger};
use logging_support::RecordingLogger;
use std::sync::Arc;
use std::thread;

/// Each required method dispatches its exact borrowed text through `&dyn Logger`.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T05, LOG-T09; Layout: F06; Test size: MD.
#[test]
fn distinct_borrowed_message_dispatch_success() {
    let recording = RecordingLogger::default();
    let logger: &dyn Logger = &recording;
    logger.info("");
    logger.warn(" \t");
    logger.error("é水🦀");
    assert_eq!(
        recording.snapshot(),
        vec![
            ("info", String::new()),
            ("warn", " \t".to_owned()),
            ("error", "é水🦀".to_owned()),
        ]
    );
}

/// Arc trait objects preserve every concurrent call; order is intentionally unspecified.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T05, LOG-T09; Layout: F06; Test size: MD.
#[test]
fn concurrent_arc_dispatch_success() {
    let recording = Arc::new(RecordingLogger::default());
    let logger: Arc<dyn Logger> = recording.clone();
    let threads: Vec<_> = (0..4)
        .map(|_| {
            let shared = Arc::clone(&logger);
            thread::spawn(move || {
                shared.info("shared");
                shared.warn("shared");
                shared.error("shared");
            })
        })
        .collect();
    for handle in threads {
        handle
            .join()
            .expect("recording Logger worker must return normally");
    }
    let mut actual = recording.snapshot();
    actual.sort();
    let mut expected: Vec<_> = (0..4)
        .flat_map(|_| {
            [
                ("info", "shared".to_owned()),
                ("warn", "shared".to_owned()),
                ("error", "shared".to_owned()),
            ]
        })
        .collect();
    expected.sort();
    assert_eq!(actual, expected);
}

/// Both explicit no-op construction paths coerce to the canonical public trait.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T01, LOG-T09; Layout: F06; Test size: MD.
#[test]
fn explicit_no_op_public_construction_success() {
    let instances: [Arc<dyn Logger>; 2] =
        [Arc::new(NoOpLogger::new()), Arc::new(NoOpLogger::default())];
    for logger in instances {
        let _: () = logger.info("ordinary");
        let _: () = logger.warn("");
        let _: () = logger.error("水");
    }
}
