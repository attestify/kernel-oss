use crate::error::Error;
use crate::ulid::ULID;

/// Defines the behavior for an implementation which provides a [ULID] which are unique identifier be used as identities for persistable entities.
pub trait IdentityGateway:  IdentityGatewayClone + Sync + Send {
    
    /// Generates a new & unique [ULID]
    fn generate(&self) -> Result<ULID, Error>;

}


/// This trait enables cloning of `Box<dyn IdentityGateway>` trait objects.
///
/// This is necessary because the `IdentityGateway` trait itself doesn't require the `Clone` trait, and trait objects with non-auto traits (like `Clone`)  can't be cloned directly.
///
pub trait IdentityGatewayClone {
    fn clone_box(&self) -> Box<dyn IdentityGateway>;
}


/// Blanket implementation of `IdentityGatewayClone` for any type `T` that  implements `IdentityGateway` and `Clone`.
///
/// This allows cloning of `Box<dyn IdentityGateway>` as long as the underlying concrete type implements `Clone`
///
impl<Gw>  IdentityGatewayClone for Gw
where
    Gw: 'static + IdentityGateway + Clone
{
    fn clone_box(&self) -> Box<dyn  IdentityGateway> {
        Box::new(self.clone())
    }
}

/// Implements `Clone` for `Box<dyn IdentityGateway>`.
///
/// This leverages the `IdentityGatewayClone` trait to enable cloning of the  trait object.
///
impl Clone for Box<dyn IdentityGateway> {
    fn clone(&self) -> Box<dyn IdentityGateway> {
        self.clone_box()
    }
}