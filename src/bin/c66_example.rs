// Arc<T> = Atomically Reference-Counted: share read-only data across THREADS. It's Rc (c31)
// with thread-safe atomic counts — plain Rc isn't safe to send between threads. Coming from
// C/ThreadX: a refcounted shared buffer whose count is bumped with atomic ops, so several
// threads can hold it and the last one out frees it.
//
// THE VAULT RUN: one vault map, whole crew. Mai, Linh, and Trang each get an Arc handle
// and independently tally the haul — nobody copies the map, nobody frees it early.
use std::sync::Arc;
use std::thread;

fn main() {
    let vault_map = Arc::new(vec![4000u32, 6500, 3500]); // marked caches, in creds

    let mut crew = Vec::new();
    for handle in ["Mai", "Linh", "Trang"] {
        let map = Arc::clone(&vault_map); // each crew member gets a handle
        crew.push(thread::spawn(move || {
            let estimate: u32 = map.iter().sum();
            println!("[{handle}] tallies {estimate} creds");
            estimate
        }));
    }

    let combined: u32 = crew.into_iter().map(|h| h.join().unwrap()).sum();
    println!("[crew] combined estimates: {combined} creds");
}
