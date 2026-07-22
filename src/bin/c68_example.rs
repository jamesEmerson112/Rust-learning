// RwLock<T> = a reader/writer lock: many readers OR one writer at a time (a Mutex allows only
// one accessor, period). Reach for it when reads dominate and you want them to run concurrently.
// Coming from ThreadX: a readers-writer lock — read() takes shared access, write() exclusive.
//
// THE VAULT RUN: the alert board. Three lookouts poll it constantly (shared reads);
// only the spotter ever writes. Mutex would make the lookouts queue — RwLock lets
// them all read at once.
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let board = Arc::new(RwLock::new(vec!["all-quiet".to_string()]));

    // The spotter posts an update (exclusive access).
    {
        let board = Arc::clone(&board);
        let spotter = thread::spawn(move || {
            board.write().unwrap().push("guards-rotating".to_string());
        });
        spotter.join().unwrap();
    }

    // Lookouts read concurrently — many readers can hold the lock at once.
    let mut lookouts = Vec::new();
    for handle in ["Mai", "Linh", "Trang"] {
        let board = Arc::clone(&board);
        lookouts.push(thread::spawn(move || {
            let alerts = board.read().unwrap();
            println!("[{handle}] sees {} alerts", alerts.len());
            alerts.len()
        }));
    }
    for l in lookouts {
        l.join().unwrap();
    }
}
