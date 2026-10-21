//! Declaration-only operational logging contract.
//!
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: seam
//! Targets: LOG-T05, LOG-T08, LOG-T09, LOG-T11
//! Layout: F03

/// A shared, synchronous operational logging interface.
///
/// Contract: kernel-oss-operational-logger@0.1.0
/// Role: seam
/// Targets: LOG-T05, LOG-T08, LOG-T09, LOG-T11
/// Layout: F03
///
/// All three methods are required. They borrow a message and return unit;
/// normal return is not an acknowledgment that a destination received it.
/// There is no shared initialization hook, default method body, asynchronous
/// operation, global installation, or `Error`/`Result` delivery channel.
/// Implementations can be shared through `&dyn Logger` or `Arc<dyn Logger>`.
/// A borrowed argument alone does not prevent a concrete logger from copying it.
///
/// Concrete implementations own synchronization, safe preparation, recovery,
/// sanitized messages and finite supported wait/retry policies. Logging belongs
/// on decorators or separate composition components, not as retained state in
/// core Usecases/Gateways. It must not replace business results or required
/// event delivery. This trait promises neither nonblocking calls nor immunity
/// from arbitrary implementation panics or allocation failure.
///
/// # Destination failures and diagnostics
///
/// The accepted local policy is in the crate package's `docs/logging.md`,
/// "Concrete implementation obligations", "Health diagnostics, never payload
/// fallback", and "Episodes, concurrency and bounded execution". Concrete
/// implementations abandon failed original messages; emergency routes report
/// logger health only and must never replay original messages or fields.
/// Follow that policy's eligible independent routes, closed safe fields,
/// bounded counters, episode accounting and timing. It is a local Kernel
/// contract documented entirely within this crate. Kernel supplies no
/// backend or diagnostic API, and no-op tests do not qualify concrete sinks.
/// [`super::NoOpLogger`] performs no delivery and emits no failure diagnostics.
///
/// # Example
///
/// Exercises public construction, sharing and all three unit-returning calls
/// (LOG-T01, LOG-T02, LOG-T03, LOG-T04, LOG-T09):
///
/// ```
/// use kernel_oss::logging::{Logger, NoOpLogger};
/// use std::sync::Arc;
///
/// let logger: Arc<dyn Logger> = Arc::new(NoOpLogger::new());
/// logger.info("operation completed");
/// logger.warn("");
/// logger.error("operation unavailable");
/// NoOpLogger::default().info("discarded");
/// ```
pub trait Logger: Send + Sync {
    /// Reports operational information; unit return is not a delivery receipt.
    fn info(&self, message: &str);

    /// Reports an operational warning; unit return is not a delivery receipt.
    fn warn(&self, message: &str);

    /// Reports an operational failure; unit return is not a delivery receipt.
    fn error(&self, message: &str);
}
