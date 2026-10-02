// RwLock<T> = a reader/writer lock: many readers OR one writer at a time (a Mutex allows only
// one accessor, period). Reach for it when reads dominate and you want them to run concurrently.
// Coming from ThreadX: a readers-writer lock — read() takes shared access, write() exclusive.
//
// RUST GENERAL HOSPITAL: the ward's bed board. Three nurses check it constantly (shared
// reads); only the charge nurse ever updates it. Mutex would make the nurses queue —
// RwLock lets them all read at once.
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let board = Arc::new(RwLock::new(vec!["bed-4 free".to_string()]));

    // The charge nurse posts an update (exclusive access).
    {
        let board = Arc::clone(&board);
        let charge_nurse = thread::spawn(move || {
            board.write().unwrap().push("bed-9 free".to_string());
        });
        charge_nurse.join().unwrap();
    }

    // Nurses read concurrently — many readers can hold the lock at once.
    let mut readers = Vec::new();
    for nurse in ["Mai", "Linh", "Trang"] {
        let board = Arc::clone(&board);
        readers.push(thread::spawn(move || {
            let free_beds = board.read().unwrap();
            println!("[{nurse}] sees {} free beds", free_beds.len());
            free_beds.len()
        }));
    }
    for r in readers {
        r.join().unwrap();
    }
}
