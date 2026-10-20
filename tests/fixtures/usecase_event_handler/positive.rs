use handler_domain::{Event, OptionalHandler, RequiredHandler};
use kernel_oss::{
    error::Error,
    usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler},
};

struct Real;
impl UsecaseEventHandler for Real {
    type Event = Event;
    fn handle(&self, _: Event) -> Result<(), Error> {
        Ok(())
    }
}
impl RequiredHandler for Real {}

fn main() {
    let optional: &dyn OptionalHandler = &NoOpUsecaseEventHandler::<Event>::new();
    let required: &dyn RequiredHandler = &Real;
    assert_eq!(optional.handle(Event), Ok(()));
    assert_eq!(required.handle(Event), Ok(()));
    assert_eq!(NoOpUsecaseEventHandler::<u32>::new().handle(1), Ok(()));
}
