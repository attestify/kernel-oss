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
