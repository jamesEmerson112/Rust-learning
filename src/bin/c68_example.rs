// RwLock<T> = a reader/writer lock: many readers OR one writer at a time (a Mutex allows only
// one accessor, period). Reach for it when reads dominate and you want them to run concurrently.
// Coming from ThreadX: a readers-writer lock — read() takes shared access, write() exclusive.
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let schedule = Arc::new(RwLock::new(vec!["Mai - 10:00".to_string()]));

    // One writer adds an appointment (exclusive access).
    {
        let schedule = Arc::clone(&schedule);
        let writer = thread::spawn(move || {
            schedule.write().unwrap().push("Linh - 10:30".to_string());
        });
        writer.join().unwrap();
    }

    // Many readers can hold the lock at the same time.
    let mut handles = Vec::new();
    for _ in 0..3 {
        let schedule = Arc::clone(&schedule);
        handles.push(thread::spawn(move || schedule.read().unwrap().len()));
    }
    for h in handles {
        println!("reader sees {} appointments", h.join().unwrap());
    }
}
