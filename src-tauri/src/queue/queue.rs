use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use tokio::task::JoinSet;

pub(crate) struct Queue<T> {
    state: Mutex<QueueState<T>>,
    available: Condvar,
}

pub(crate) struct QueueState<T> {
    pending: VecDeque<T>,
    current_jobs: usize,
    cancelled: bool
}

impl<T> QueueState<T> {
    pub(crate) fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            current_jobs: 0,
            cancelled: false,
        }
    }
}

impl<T> Queue<T> {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(QueueState::new()),
            available: Default::default(),
        }
    }

    pub(crate) fn push(&self, item: T) {
        self.state.lock().unwrap().pending.push_back(item);
        self.available.notify_one();
    }

    pub(crate) fn cancel(&self) {
        self.state.lock().unwrap().cancelled = true;
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
                        let mut state = queue.state.lock().unwrap();

                        loop {
                            if state.cancelled {
                                return;
                            }

                            if let Some(item) = state.pending.pop_front() {
                                state.current_jobs += 1;

                                break item;
                            }

                            if state.current_jobs == 0 {
                                return;
                            }
                            state = queue.available.wait(state).unwrap();
                        }
                    };

                    closure(item);

                    queue.state.lock().unwrap().current_jobs -= 1;

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
