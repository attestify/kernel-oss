//! Generic conversion-style traits shared across the kernel.

/// Converts a value into another type.
pub trait To<T> {
    /// Performs the conversion.
    fn to(&self) -> T;
}
