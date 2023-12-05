mod executor;
mod task;
mod uid_generator;
mod uid_waker;
mod util;

/// Execute tasks from the thread-local queue until all tasks are completed.
/// The callback `on_idle` is called when all tasks are blocked.
pub fn run(mut on_idle: impl FnMut()) {
    while executor::poll_until_idle() {
        on_idle();
    }
}

/// Put a task into the thread-local queue.
pub fn spawn(future: impl std::future::Future<Output = ()> + 'static) {
    let task = task::Task::new(future);
    executor::add_to_poll(task);
}
