// RwLock<T> is a reader-writer lock. At any moment it allows either many readers or one
// writer. A Mutex allows only one thread at a time, whether it reads or writes. Use RwLock
// when reads far outnumber writes and you want the reads to run at the same time.
// Coming from C: in ThreadX terms, this is a readers-writer lock. read() takes shared
// access, and write() takes exclusive access.
//
// RUST GENERAL HOSPITAL: the ward has a bed board. Three nurses read it constantly, and only
// the charge nurse ever updates it. With a Mutex, the nurses would wait in line to read it.
// With an RwLock, they can all read it at once.
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let board = Arc::new(RwLock::new(vec!["bed-4 free".to_string()]));

    // The charge nurse posts an update, which needs exclusive access.
    {
        let board = Arc::clone(&board);
        let charge_nurse = thread::spawn(move || {
            board.write().unwrap().push("bed-9 free".to_string());
        });
        charge_nurse.join().unwrap();
    }

    // The nurses read at the same time, because many readers can hold the lock at once.
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
