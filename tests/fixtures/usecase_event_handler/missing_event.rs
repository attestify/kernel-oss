use kernel_oss::{error::Error, usecase_event_handler::UsecaseEventHandler};
struct Handler;
impl UsecaseEventHandler for Handler {
    fn handle(&self, _: ()) -> Result<(), Error> {
        Ok(())
    }
}
fn main() {}
