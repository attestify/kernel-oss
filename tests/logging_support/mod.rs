//! Test-only Logger fixture exports.
//! Contract: kernel-oss-operational-logger@0.1.0
//! Role: test; Targets: LOG-T05; Layout: F14.

mod value;

pub(crate) use value::RecordingLogger;
