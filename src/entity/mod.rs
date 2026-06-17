/// Exposes the stable identity held by an entity.
pub trait Entity {
    /// The bounded identity type for the entity.
    type IdType;

    /// Returns the entity identity.
    fn id(&self) -> &Self::IdType;
}
