//! Reusable bounded value objects and value role traits.
//!
//! Use [`Value`] when a type exposes one canonical bounded value. The
//! submodules contain reusable kernel values for time, text, file-system paths,
//! namespace resource names, URLs, and Attestify specification data.

/// Copy-on-demand value helpers.
pub mod copy_value;
pub mod datetime;
pub mod directory;
pub mod file_system;
pub mod nrn;
pub mod specification;
/// String bounded values.
pub mod strings;
pub mod text;
pub mod uri;

#[cfg(test)]
mod strings_tests;

/// Exposes the canonical bounded value held by a value object.
///
/// Implement this trait when a value object has one primary value that defines
/// its equality, hashing, display, or conversion contract.
pub trait Value {
    /// The canonical value type exposed by the value object.
    type ValueType: ?Sized;

    /// Returns the canonical bounded value.
    fn value(&self) -> &Self::ValueType;
}
