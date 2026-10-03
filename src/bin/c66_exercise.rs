// RUST GENERAL HOSPITAL — Night Shift
// Three nurses share one supply list, and each nurse runs on her own thread. Each nurse
// checks the count independently. Share the list between the threads instead of copying it.
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::thread;

pub fn supply_check() -> u32 {
    // TODO: Wrap the supply list vec![4000u32, 6500, 3500] in an Arc. Spawn 3 threads
    // that each take a handle with Arc::clone and sum the list. Join all three and return
    // the grand total of their counts, which is 3 × 14000 = 42000.
    // Plain Rc would not compile here, because it is not Send, the marker trait for types
    // that can be moved to another thread. Arc is Send.
    0
}

fn main() {
    println!("[ward] all three counts together: {} (want 42000)", supply_check());
}
