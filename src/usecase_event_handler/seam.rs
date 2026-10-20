use crate::error::Error;

/// Defines synchronous typed delivery, distinct from Usecase and Gateway execution.
///
/// Domain markers fix [`Self::Event`] to a finalized parent Response or dedicated
/// Event. The associated type has no universal `Send` or `Sync` bound: each
/// concrete contract owns its input's concurrency requirements. The Handler
/// itself must support safe concurrent sharing.
///
/// Implementations must not retain invocation input after the call. Stable
/// output resources and their synchronization belong to the concrete Handler's
/// contract. This trait supplies no routing, retry, or asynchronous delivery.
///
/// ```
/// use kernel_oss::{error::Error, usecase_event_handler::UsecaseEventHandler};
///
/// struct Delivery;
/// impl UsecaseEventHandler for Delivery {
///     type Event = ();
///     fn handle(&self, event: ()) -> Result<(), Error> { Ok(()) }
/// }
/// let handler: &dyn UsecaseEventHandler<Event = ()> = &Delivery;
/// assert_eq!(handler.handle(()), Ok(()));
/// ```
///
/// The input must match the associated type:
///
/// ```compile_fail,E0308
/// use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
/// NoOpUsecaseEventHandler::<u32>::new().handle("wrong input");
/// ```
///
/// Every implementation must declare its input:
///
/// ```compile_fail,E0046
/// use kernel_oss::{error::Error, usecase_event_handler::UsecaseEventHandler};
/// struct MissingInput;
/// impl UsecaseEventHandler for MissingInput {
///     fn handle(&self, _: ()) -> Result<(), Error> { Ok(()) }
/// }
/// ```
///
/// A marker implementation alone cannot supply the shared base:
///
/// ```compile_fail,E0277
/// use kernel_oss::usecase_event_handler::UsecaseEventHandler;
/// trait LocalMarker: UsecaseEventHandler<Event = ()> {}
/// struct MissingBase;
/// impl LocalMarker for MissingBase {}
/// ```
pub trait UsecaseEventHandler: Send + Sync {
    /// Exact finalized input consumed by this Handler.
    type Event;

    /// Consumes the finalized input and performs the contracted delivery.
    ///
    /// # Errors
    ///
    /// Returns the concrete Handler's bounded Kernel Error when delivery fails.
    /// The parent owns propagation or continuation policy. Unit success means
    /// completion of the selected contract, including explicitly permitted
    /// no-op completion; it does not universally prove boundary output.
    fn handle(&self, event: Self::Event) -> Result<(), Error>;
}
