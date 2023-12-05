use crate::task::Task;

thread_local! {
    static WILL_POLL: std::cell::RefCell<std::collections::VecDeque<Task>> = std::cell::RefCell::new(std::collections::VecDeque::new());
    static WILL_WAKE: std::cell::RefCell<std::collections::HashMap<usize, Task>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

fn will_poll<R>(f: impl FnOnce(&mut std::collections::VecDeque<Task>) -> R) -> R {
    WILL_POLL.with_borrow_mut(f)
}

fn will_wake<R>(f: impl FnOnce(&mut std::collections::HashMap<usize, Task>) -> R) -> R {
    WILL_WAKE.with_borrow_mut(f)
}

pub struct ToPoll;
pub struct ToWake;

impl ToPoll {
    pub fn push_back(task: Task) {
        will_poll(|v| v.push_back(task));
    }

    pub fn pop_front() -> Option<Task> {
        will_poll(|v| v.pop_front())
    }
}

impl ToWake {
    pub fn insert(id: usize, task: Task) -> Option<Task> {
        will_wake(|v| v.insert(id, task))
    }

    pub fn remove(id: usize) -> Option<Task> {
        will_wake(|v| v.remove(&id))
    }

    pub fn is_empty() -> bool {
        will_wake(|v| v.is_empty())
    }
}
