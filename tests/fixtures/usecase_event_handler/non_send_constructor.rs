use kernel_oss::usecase_event_handler::NoOpUsecaseEventHandler;
use std::rc::Rc;
fn main() {
    let _ = NoOpUsecaseEventHandler::<Rc<()>>::new();
}
