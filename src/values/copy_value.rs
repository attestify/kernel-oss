use crate::error::Error;

pub trait CopyValue  {
    fn copy(&self) -> Result<Self, Error> where Self: Sized;
}