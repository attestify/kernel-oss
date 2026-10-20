use super::UsecaseEventHandler;
use crate::error::Error;
use std::marker::PhantomData;

#[cfg(test)]
mod tests;

/// A stateless Handler for explicitly permitted optional delivery.
///
/// Construction and handling require `Input: Send + Sync`. The no-op stores no
/// input or output resource and performs no delivery. The owning Domain crate
/// may implement its marker for this type when its parent contract permits it.
/// Application must explicitly select it; `Default` is not a dependency fallback.
///
/// ```
/// use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
/// let handler = NoOpUsecaseEventHandler::<String>::new();
/// assert_eq!(handler.handle(String::from("finalized input")), Ok(()));
/// let default_handler = NoOpUsecaseEventHandler::<String>::default();
/// assert_eq!(default_handler.handle(String::new()), Ok(()));
/// ```
///
/// Inputs lacking the required bounds cannot use this constructor:
///
/// ```compile_fail,E0599
/// use kernel_oss::usecase_event_handler::NoOpUsecaseEventHandler;
/// use std::rc::Rc;
/// let handler = NoOpUsecaseEventHandler::<Rc<()>>::new();
/// ```
pub struct NoOpUsecaseEventHandler<Input> {
    marker: PhantomData<fn(Input)>,
}

impl<Input: Send + Sync> NoOpUsecaseEventHandler<Input> {
    /// Constructs an explicit no-op with no configuration or resources.
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<Input: Send + Sync> Default for NoOpUsecaseEventHandler<Input> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Input: Send + Sync> UsecaseEventHandler for NoOpUsecaseEventHandler<Input> {
    type Event = Input;

    /// Drops the input before returning unit success without performing output.
    ///
    /// # Errors
    ///
    /// Never returns a delivery Error because no delivery is attempted.
    ///
    /// # Panics
    ///
    /// A panic in the input's destructor propagates; it is not converted to an
    /// Error or successful completion.
    fn handle(&self, input: Input) -> Result<(), Error> {
        drop(input);
        Ok(())
    }
}
