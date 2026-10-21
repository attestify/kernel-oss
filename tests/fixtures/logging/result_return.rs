//! Intentional `result_return` compiler rejection through the actual Logger trait.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F13.

#[path = "value/result_return/mod.rs"]
mod result_return;

/// Compilation must reject the included implementation before any execution.
fn main() {}
