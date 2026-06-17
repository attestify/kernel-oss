//! Shared kernel primitives and seam traits for Attestify Rust crates.
//!
//! `kernel_oss` provides bounded errors, reusable value objects, entity role
//! traits, synchronous and asynchronous use case roles, and gateway seam roles.
//! Domain and application crates should prefer these primitives before
//! introducing duplicate request, response, value, entity, use case, or gateway
//! abstractions.
//!
//! Start with the human reuse catalog when deciding whether the kernel already
//! contains the type or seam you need:
//!
//! - `docs/kernel-catalog.md` in the crate package
//! - the repository README
//! - the checked examples under `examples/`
//!
//! The catalog is curated guidance; rustdoc remains the generated API reference.

pub mod algorithms;
pub mod core;
pub mod entity;
pub mod error;
pub mod gateway;
pub mod response;
pub mod ulid;
pub mod usecase;
pub mod values;
