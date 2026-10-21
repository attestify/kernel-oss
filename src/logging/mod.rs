//! Synchronous operational logging and an explicitly constructed no-op.
//!
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: module
//! Targets: LOG-T09
//! Layout: F02
//!
//! These are operational interfaces, separate from Gateway and business-event
//! delivery. The crate package's `docs/logging/README.md` owns the consumer contract
//! and concrete implementation obligations.

mod no_op_logger;
mod seam;

pub use no_op_logger::NoOpLogger;
pub use seam::Logger;
