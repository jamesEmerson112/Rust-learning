#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread;

pub fn concurrent_revenue() -> u32 {
    // TODO: Share a counter across 10 threads, each adding 100, safely.
    // Arc shares ownership; Mutex makes the mutation thread-safe (lock first).
    // This is the threaded sibling of c44's Rc<RefCell<T>>.
    0
}

fn main() {
    println!("Revenue: {}", concurrent_revenue());
}
