//! Cancellation registry shared between the run command and the UI's cancel action.
//!
//! Handles are recovered from mutex poisoning rather than panicking — the
//! execution layer must never take the host app down (practice what we teach).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default)]
struct Inner {
    cancelled: AtomicBool,
}

#[derive(Default)]
pub struct CancelHandle(Inner);

impl CancelHandle {
    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::SeqCst);
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.cancelled.load(Ordering::SeqCst)
    }
}

#[derive(Default)]
pub struct ExecutionRegistry {
    handles: Mutex<HashMap<String, Arc<CancelHandle>>>,
}

impl ExecutionRegistry {
    fn lock(&self) -> MutexGuard<'_, HashMap<String, Arc<CancelHandle>>> {
        match self.handles.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// Register a live execution; returns its cancel handle.
    pub fn register(&self, id: String) -> Arc<CancelHandle> {
        let handle = Arc::new(CancelHandle::default());
        self.lock().insert(id, handle.clone());
        handle
    }

    /// Signal cancellation. Returns false when the id is unknown/already gone.
    pub fn cancel(&self, id: &str) -> bool {
        match self.lock().get(id).cloned() {
            Some(handle) => {
                handle.cancel();
                true
            }
            None => false,
        }
    }

    pub fn unregister(&self, id: &str) {
        self.lock().remove(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_unknown_id_returns_false() {
        let reg = ExecutionRegistry::default();
        assert!(!reg.cancel("missing"));
    }

    #[test]
    fn registered_execution_can_be_cancelled_once_registered() {
        let reg = ExecutionRegistry::default();
        let handle = reg.register("run-1".into());
        assert!(!handle.is_cancelled());
        assert!(reg.cancel("run-1"));
        assert!(handle.is_cancelled());
        reg.unregister("run-1");
        assert!(!reg.cancel("run-1"), "unregistered ids are no longer cancellable");
    }
}
