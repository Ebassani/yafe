use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize};

pub(crate) struct Queue<T> {
    pending: Mutex<VecDeque<T>>,
    available: Condvar,
    current_jobs: AtomicUsize,
    cancelled: AtomicBool
}

impl<T> Queue<T> {
    pub(crate) fn new() -> Self {
        Self {
            pending: Mutex::new(VecDeque::new()),
            available: Default::default(),
            current_jobs: Default::default(),
            cancelled: Default::default(),
        }
    }

    pub(crate) fn push(&self, item: T) {
        self.pending.lock().unwrap().push_back(item);
        self.available.notify_one();
    }

    pub(crate) fn pop_front(&self) -> Option<T> {
        self.pending.lock().unwrap().pop_front()
    }
}
