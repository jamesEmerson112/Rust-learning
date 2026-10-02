// RUST GENERAL HOSPITAL — Night Shift — ★ BUG HUNT ★
//
// BUG: Ten nurses each gave 100 ml and every one of them charted it. The fluid-balance
// total reads 0 ml. The entries vanish somewhere between the lock and the chart.
// (The compiler is even waving a clue at you: `cargo build` warns about this file.)
//
// Find it, fix it: cargo test --test c67_tests
use std::sync::{Arc, Mutex};
use std::thread;

pub fn fluid_total() -> u32 {
    let fluid_ml = Arc::new(Mutex::new(0u32));

    let mut nurses = Vec::new();
    for _ in 0..10 {
        let fluid_ml = Arc::clone(&fluid_ml);
        nurses.push(thread::spawn(move || {
            let mut charted = *fluid_ml.lock().unwrap();
            charted += 100;
        }));
    }

    for nurse in nurses {
        nurse.join().unwrap();
    }

    let total = *fluid_ml.lock().unwrap();
    total
}

fn main() {
    println!("[chart] fluid given today: {} ml (want 1000)", fluid_total());
}
