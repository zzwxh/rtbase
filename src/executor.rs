use crate::{
    task::Task,
    uid_generator::UidGenerator,
    uid_waker::{UidWake, UidWaker},
    util::{ToPoll, ToWake},
};

pub fn add_to_poll(task: Task) {
    ToPoll::push_back(task);
}

pub fn poll_until_idle() -> bool {
    while let Some(mut task) = ToPoll::pop_front() {
        let id = UidGenerator::generate();
        let waker = UidWaker::<OnWake>::create(id);
        if task.poll(&waker).is_pending() {
            let o = ToWake::insert(id, task);
            if o.is_some() {
                eprintln!("Error task ID conflict.");
                std::process::exit(1);
            }
        }
    }
    !ToWake::is_empty()
}

struct OnWake;

impl UidWake for OnWake {
    fn wake(id: usize) {
        if let Some(task) = ToWake::remove(id) {
            ToPoll::push_back(task);
        } else {
            eprintln!("Error task ID not found.");
            std::process::exit(1);
        }
    }
}
