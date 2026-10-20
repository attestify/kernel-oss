//! Verifies the actual no-op's construction, destruction and shared-call contract.

use super::{NoOpUsecaseEventHandler, UsecaseEventHandler};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct DropProbe(Arc<AtomicUsize>);

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

/// Both explicit constructors return unit success without requiring Input: Default.
#[test]
fn explicit_constructors_return_unit_success() {
    let drops = Arc::new(AtomicUsize::new(0));
    let handlers = [
        NoOpUsecaseEventHandler::<DropProbe>::new(),
        NoOpUsecaseEventHandler::<DropProbe>::default(),
    ];
    for (index, handler) in handlers.iter().enumerate() {
        let result = handler.handle(DropProbe(Arc::clone(&drops)));
        assert_eq!(result, Ok(()));
        assert_eq!(drops.load(Ordering::SeqCst), index + 1);
        assert_eq!(Arc::strong_count(&drops), 1);
    }
}

/// Every call destroys its own input once before returning, without retention.
#[test]
fn repeated_calls_drop_each_input_before_return_success() {
    let drops = Arc::new(AtomicUsize::new(0));
    let handler = NoOpUsecaseEventHandler::new();
    for expected in 1..=8 {
        assert_eq!(handler.handle(DropProbe(Arc::clone(&drops))), Ok(()));
        assert_eq!(drops.load(Ordering::SeqCst), expected);
        assert_eq!(Arc::strong_count(&drops), 1);
    }
}

/// Concurrent calls share the same Handler but retain none of their inputs.
#[test]
fn concurrent_shared_calls_release_all_inputs_success() {
    let handler = Arc::new(NoOpUsecaseEventHandler::new());
    let drops = Arc::new(AtomicUsize::new(0));
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let handler = Arc::clone(&handler);
            let drops = Arc::clone(&drops);
            scope.spawn(move || {
                for _ in 0..16 {
                    assert_eq!(handler.handle(DropProbe(Arc::clone(&drops))), Ok(()));
                }
            });
        }
    });
    assert_eq!(drops.load(Ordering::SeqCst), 128);
    assert_eq!(Arc::strong_count(&drops), 1);
}

/// An input destructor panic is observable and never reported as unit success.
#[test]
#[cfg(panic = "unwind")]
fn destructor_panic_propagates() {
    struct PanickingInput;
    impl Drop for PanickingInput {
        fn drop(&mut self) {
            panic!("input destructor panic");
        }
    }
    let handler = NoOpUsecaseEventHandler::new();
    let outcome = std::panic::catch_unwind(|| handler.handle(PanickingInput));
    let panic = outcome.expect_err("the destructor must unwind out of handle");
    assert_eq!(
        panic.downcast_ref::<&str>(),
        Some(&"input destructor panic")
    );
}
