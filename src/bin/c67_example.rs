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
