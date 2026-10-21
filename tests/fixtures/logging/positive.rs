//! Actual public no-op constructor/call control; must run without stdout/stderr.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T01–LOG-T04, LOG-T07–LOG-T09; Layout: F08.
//! No silence assertion here: the parent harness observes this binary's streams.

use kernel_oss::logging::{Logger, NoOpLogger};
use std::sync::Arc;

/// Exercises both constructors and direct/dynamic methods without keeping borrows.
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: test; Targets: LOG-T01–LOG-T04, LOG-T07–LOG-T09; Layout: F08.
fn main() {
    let direct = NoOpLogger::new();
    let _: () = direct.info("");
    let _: () = direct.warn(" \t");
    let _: () = direct.error("é水🦀");
    let default = NoOpLogger::default();
    let _: () = default.info("");
    let _: () = default.warn(" \t");
    let _: () = default.error("é水🦀");
    let logger: Arc<dyn Logger> = Arc::new(default);
    let borrowed: &dyn Logger = logger.as_ref();
    let mut text = String::from("borrowed");
    let _: () = borrowed.info(&text);
    let _: () = borrowed.warn(&text);
    let _: () = borrowed.error(&text);
    text.clear();
    drop(text);
    logger.info("after input destruction");
}
