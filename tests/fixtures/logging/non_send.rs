//! Intentional `non_send` compiler rejection through the actual Logger trait.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F10.

#[path = "value/non_send/mod.rs"]
mod non_send;

/// Compilation must reject the included implementation before any execution.
fn main() {}
