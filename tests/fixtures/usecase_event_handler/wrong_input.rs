use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
fn main() {
    let _ = NoOpUsecaseEventHandler::<u32>::new().handle("not a u32");
}
