use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
use std::cell::Cell;
fn require_handler<H: UsecaseEventHandler>() {}
fn main() {
    require_handler::<NoOpUsecaseEventHandler<Cell<u32>>>();
}
