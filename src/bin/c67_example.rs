// To share MUTABLE state across threads, wrap it: Arc<Mutex<T>>. Arc shares ownership, Mutex
// guards the data — you must lock() to touch it, and the data is unreachable without the lock,
// so you can't forget. Coming from ThreadX: tx_mutex_get/tx_mutex_put around a shared variable,
// but here the lock is welded to the data. This is the threaded mirror of c44's Rc<RefCell>.
//
// THE VAULT RUN: ten runners feed cred-chips into ONE shared take. Lock, deposit through
// the guard, release. The ledger can't lie if every write goes through the lock.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let take = Arc::new(Mutex::new(0u32));

    let mut crew = Vec::new();
    for _ in 0..10 {
        let take = Arc::clone(&take);
        crew.push(thread::spawn(move || {
            *take.lock().unwrap() += 100; // deposit THROUGH the guard
        }));
    }

    for runner in crew {
        runner.join().unwrap();
    }

    println!("[stash] the take: {} creds", *take.lock().unwrap());
}
