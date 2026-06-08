#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::thread;

pub fn shared_across_threads() -> u32 {
    // TODO: Wrap a price list in an Arc, spawn 3 threads that each Arc::clone
    // it and sum the prices, then join all threads and return the grand total.
    // (Plain Rc would NOT compile here — it isn't Send across threads; Arc is.)
    0
}

fn main() {
    println!("Total across threads: {}", shared_across_threads());
}
