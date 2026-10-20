//! An actual separate crate owns the markers and the permitted no-op binding.
use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};

pub struct Event;
pub trait OptionalHandler: UsecaseEventHandler<Event = Event> {}
pub trait RequiredHandler: UsecaseEventHandler<Event = Event> {}
pub trait OtherHandler: UsecaseEventHandler<Event = Event> {}

impl OptionalHandler for NoOpUsecaseEventHandler<Event> {}
