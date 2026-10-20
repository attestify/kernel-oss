use handler_domain::{Event, RequiredHandler};
use kernel_oss::usecase_event_handler::NoOpUsecaseEventHandler;

// This crate owns neither the Domain marker nor the Kernel no-op.
impl RequiredHandler for NoOpUsecaseEventHandler<Event> {}
fn main() {}
