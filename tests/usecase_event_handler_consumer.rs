//! Public consumer coverage for typed delivery, Domain binding and Error identity.

use kernel_oss::error::{
    Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};
use kernel_oss::usecase_event_handler::{NoOpUsecaseEventHandler, UsecaseEventHandler};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

struct ReadEvent(u32);
trait ReadHandler: UsecaseEventHandler<Event = ReadEvent> {}
trait AuditHandler: UsecaseEventHandler<Event = ReadEvent> {}
impl ReadHandler for NoOpUsecaseEventHandler<ReadEvent> {}
impl AuditHandler for NoOpUsecaseEventHandler<ReadEvent> {}

struct ReadResponse(String);
trait ResponseHandler: UsecaseEventHandler<Event = ReadResponse> {}
impl ResponseHandler for NoOpUsecaseEventHandler<ReadResponse> {}

struct Output(Mutex<Vec<u32>>);
impl UsecaseEventHandler for Output {
    type Event = ReadEvent;

    fn handle(&self, event: ReadEvent) -> Result<(), Error> {
        self.0
            .lock()
            .map_err(|_| Error::for_system(Kind::ProcessingFailure, "Output lock is poisoned."))?
            .push(event.0);
        Ok(())
    }
}
impl ReadHandler for Output {}

/// A real realization and the Domain-owned no-op binding share one exact seam.
#[test]
fn typed_marker_dispatches_real_and_no_op_handlers_success() {
    let output = Arc::new(Output(Mutex::new(Vec::new())));
    let real: Arc<dyn ReadHandler> = output.clone();
    let no_op: Arc<dyn ReadHandler> = Arc::new(NoOpUsecaseEventHandler::new());
    assert_eq!(real.handle(ReadEvent(7)), Ok(()));
    assert_eq!(no_op.handle(ReadEvent(9)), Ok(()));
    assert_eq!(*output.0.lock().expect("test output lock is healthy"), [7]);
}

/// Two capability markers can share an input without becoming the same trait.
#[test]
fn same_input_supports_distinct_explicit_marker_bindings_success() {
    let read: Arc<dyn ReadHandler> = Arc::new(NoOpUsecaseEventHandler::new());
    let audit: Arc<dyn AuditHandler> = Arc::new(NoOpUsecaseEventHandler::new());
    assert_eq!(read.handle(ReadEvent(1)), Ok(()));
    assert_eq!(audit.handle(ReadEvent(2)), Ok(()));
}

/// Direct-response delivery consumes a non-Clone response without another envelope.
#[test]
fn finalized_response_is_a_valid_exact_input_success() {
    let handler: &dyn ResponseHandler = &NoOpUsecaseEventHandler::new();
    let response = ReadResponse(String::from("bounded response"));
    assert_eq!(response.0, "bounded response");
    assert_eq!(handler.handle(response), Ok(()));
}

/// Safe shared Handlers need not require Send/Sync on their associated input.
#[test]
fn base_supports_non_send_input_with_synchronous_consumption_success() {
    struct LocalInputHandler;
    impl UsecaseEventHandler for LocalInputHandler {
        type Event = Rc<()>;
        fn handle(&self, input: Self::Event) -> Result<(), Error> {
            drop(input);
            Ok(())
        }
    }
    let input = Rc::new(());
    let handler: &dyn UsecaseEventHandler<Event = Rc<()>> = &LocalInputHandler;
    assert_eq!(handler.handle(Rc::clone(&input)), Ok(()));
    assert_eq!(Rc::strong_count(&input), 1);
}

/// Trait-object dispatch preserves the full returned Error, including provenance.
#[test]
fn delivery_error_preserves_error_and_provenance() {
    struct FailedDelivery(Error);
    impl UsecaseEventHandler for FailedDelivery {
        type Event = ReadEvent;
        fn handle(&self, _: ReadEvent) -> Result<(), Error> {
            Err(self.0.clone())
        }
    }
    impl ReadHandler for FailedDelivery {}

    let descriptor = |code| {
        ErrorProvenanceDescriptor::new(
            ErrorProvenanceNamespace::try_new("handler.test").expect("valid test namespace"),
            ErrorProvenanceSchemaVersion::try_new(1).expect("nonzero test version"),
            ErrorProvenanceCode::try_new(code).expect("valid test code"),
        )
    };
    let plain = Error::for_system(Kind::ProcessingFailure, "Delivery failed.");
    let attached = plain
        .try_with_attached_provenance(descriptor("origin"))
        .expect("plain error accepts provenance");
    let derived = attached
        .try_with_derived_provenance(descriptor("current"))
        .expect("attached error accepts derivation");
    for expected in [plain, attached, derived] {
        let handler: Arc<dyn ReadHandler> = Arc::new(FailedDelivery(expected.clone()));
        assert_eq!(handler.handle(ReadEvent(1)), Err(expected));
    }
}
