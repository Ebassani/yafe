use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use tokio::task::JoinSet;

pub(crate) struct Queue<T> {
    pending: Mutex<VecDeque<T>>,
    available: Condvar,
    current_jobs: AtomicUsize,
    cancelled: AtomicBool,
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

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
        self.available.notify_all();
    }

    pub(crate) async fn run_blocking<F>(self: Arc<Self>, thread_amount: usize, function: F)
    where
        T: Send + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        let thread_amount = thread_amount.max(1);

        let function = Arc::new(function);
        let mut tasks = JoinSet::new();

        for _ in 0..thread_amount {
            let queue = Arc::clone(&self);
            let closure = Arc::clone(&function);

            tasks.spawn_blocking(move || {
                loop {
                    let item = {
                        let mut pending = queue.pending.lock().unwrap();

                        loop {
                            if queue.cancelled.load(Ordering::Relaxed) {
                                return;
                            }

                            if let Some(item) = pending.pop_front() {
                                queue.current_jobs.fetch_add(1, Ordering::Relaxed);

                                break item;
                            }

                            if queue.current_jobs.load(Ordering::Relaxed) == 0 {
                                return;
                            }
                            pending = queue.available.wait(pending).unwrap();
                        }
                    };

                    closure(item);

                    queue.current_jobs.fetch_sub(1, Ordering::Relaxed);

                    queue.available.notify_all()
                }
            });
        }

        while let Some(result) = tasks.join_next().await {
            if result.is_err() {
                self.cancel();
            }
        }
    }
}
