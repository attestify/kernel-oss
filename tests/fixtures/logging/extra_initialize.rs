//! Intentional `extra_initialize` compiler rejection through the actual Logger trait.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T08; Layout: F12.

#[path = "value/extra_initialize/mod.rs"]
mod extra_initialize;

/// Compilation must reject the included implementation before any execution.
fn main() {}
