// To share mutable state across threads, wrap it in Arc<Mutex<T>>. Arc shares ownership of
// the data, and Mutex guards it. You must call lock() to reach the data, and there is no other
// way to reach it, so you cannot forget to lock. This is the threaded version of the
// Rc<RefCell<T>> pattern from c44.
// Coming from C: in ThreadX, you call tx_mutex_get and tx_mutex_put around a shared variable.
// Here the lock and the data it protects are a single object.
//
// RUST GENERAL HOSPITAL: ten nurses add the fluids they gave to one shared fluid-balance
// total. Each nurse locks the total, adds to it through the guard that lock() returns, and
// releases the lock. If every write goes through the lock, the total cannot come out wrong.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let fluid_ml = Arc::new(Mutex::new(0u32));

    let mut nurses = Vec::new();
    for _ in 0..10 {
        let fluid_ml = Arc::clone(&fluid_ml);
        nurses.push(thread::spawn(move || {
            *fluid_ml.lock().unwrap() += 100; // write through the guard
        }));
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    println!("[chart] fluid given today: {} ml", *fluid_ml.lock().unwrap());
}
