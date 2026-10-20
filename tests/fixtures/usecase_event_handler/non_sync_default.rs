use kernel_oss::usecase_event_handler::NoOpUsecaseEventHandler;
use std::cell::Cell;
fn main() {
    let _ = NoOpUsecaseEventHandler::<Cell<u32>>::default();
}
