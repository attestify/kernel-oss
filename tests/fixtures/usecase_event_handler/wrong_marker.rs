use handler_domain::{OptionalHandler, OtherHandler};
fn convert(handler: &dyn OptionalHandler) -> &dyn OtherHandler {
    handler
}
fn main() {}
