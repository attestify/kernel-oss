//! Intentional `non_sync` compiler rejection through the actual Logger trait.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F11.

#[path = "value/non_sync/mod.rs"]
mod non_sync;

/// Compilation must reject the included implementation before any execution.
fn main() {}
