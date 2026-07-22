// THE VAULT RUN — Chapter 4: THE CREW — ★ BUG HUNT ★
//
// BUG: Ten runners, 100 creds each — every one of them swears they deposited.
// The stash reads 0. The chips vanish somewhere between the lock and the ledger.
// (The compiler is even waving a clue at you: `cargo build` warns about this file.)
//
// Find it, fix it: cargo test --test c67_tests
use std::sync::{Arc, Mutex};
use std::thread;

pub fn pool_the_take() -> u32 {
    let take = Arc::new(Mutex::new(0u32));

    let mut crew = Vec::new();
    for _ in 0..10 {
        let take = Arc::clone(&take);
        crew.push(thread::spawn(move || {
            let mut chip_stack = *take.lock().unwrap();
            chip_stack += 100;
        }));
    }

    for runner in crew {
        runner.join().unwrap();
    }

    let total = *take.lock().unwrap();
    total
}

fn main() {
    println!("[stash] the take: {} creds (want 1000)", pool_the_take());
}
