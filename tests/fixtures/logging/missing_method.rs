//! Intentional `missing_method` compiler rejection through the actual Logger trait.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F09.

#[path = "value/missing_method/mod.rs"]
mod missing_method;

/// Compilation must reject the included implementation before any execution.
fn main() {}
