// To share MUTABLE state across threads, wrap it: Arc<Mutex<T>>. Arc shares ownership, Mutex
// guards the data — you must lock() to touch it, and the data is unreachable without the lock,
// so you can't forget. Coming from ThreadX: tx_mutex_get/tx_mutex_put around a shared variable,
// but here the lock is welded to the data. This is the threaded mirror of c44's Rc<RefCell>.
//
// RUST GENERAL HOSPITAL: ten nurses chart fluids given into ONE shared fluid-balance total.
// Lock, add through the guard, release. The total can't be wrong if every write goes
// through the lock.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let fluid_ml = Arc::new(Mutex::new(0u32));

    let mut nurses = Vec::new();
    for _ in 0..10 {
        let fluid_ml = Arc::clone(&fluid_ml);
        nurses.push(thread::spawn(move || {
            *fluid_ml.lock().unwrap() += 100; // chart THROUGH the guard
        }));
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    println!("[chart] fluid given today: {} ml", *fluid_ml.lock().unwrap());
}
