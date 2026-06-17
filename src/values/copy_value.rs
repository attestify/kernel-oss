use crate::error::Error;

/// A value type that can produce a validated copy of itself.
pub trait CopyValue {
    /// Returns a copied value or a validation error.
    fn copy(&self) -> Result<Self, Error>
    where
        Self: Sized;
}
