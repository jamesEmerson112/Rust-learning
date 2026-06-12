// To share MUTABLE state across threads, wrap it: Arc<Mutex<T>>. Arc shares ownership, Mutex
// guards the data — you must lock() to touch it, and the data is unreachable without the lock,
// so you can't forget. Coming from ThreadX: tx_mutex_get/tx_mutex_put around a shared variable,
// but here the lock is welded to the data. This is the threaded mirror of c44's Rc<RefCell>.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let revenue = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let revenue = Arc::clone(&revenue);
        handles.push(thread::spawn(move || {
            *revenue.lock().unwrap() += 100;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Revenue: {}", *revenue.lock().unwrap());
}
