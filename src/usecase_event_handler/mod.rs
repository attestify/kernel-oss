//! Typed synchronous delivery of finalized parent responses or events.
//!
//! Domain crates own markers fixing the exact input, and any permitted binding
//! of those markers to [`NoOpUsecaseEventHandler`]. Application composition
//! explicitly selects and injects a Handler. A no-op is suitable only where the
//! parent contract permits no delivery; its success does not establish output.
//! Parent event meaning, timing, and failure policy remain outside Kernel.
//!
//! ```
//! use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
//! use std::sync::Arc;
//!
//! struct ReadEvent;
//! trait ReadEventHandler: UsecaseEventHandler<Event = ReadEvent> {}
//! // This crate owns the marker and permits no-op delivery.
//! impl ReadEventHandler for NoOpUsecaseEventHandler<ReadEvent> {}
//!
//! let handler: Arc<dyn ReadEventHandler> =
//!     Arc::new(NoOpUsecaseEventHandler::<ReadEvent>::new());
//! assert_eq!(handler.handle(ReadEvent), Ok(()));
//! ```

mod no_op;
mod seam;

pub use no_op::NoOpUsecaseEventHandler;
pub use seam::UsecaseEventHandler;
