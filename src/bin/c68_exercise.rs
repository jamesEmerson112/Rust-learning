// RUST GENERAL HOSPITAL — Night Shift
// Three nurses read the bed board constantly, and the charge nurse writes to it.
// An RwLock allows either many readers or one writer, so reads do not wait for each other.
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};
#[allow(unused_imports)]
use std::thread;

pub fn free_beds_seen() -> usize {
    // TODO: Wrap a bed board, a Vec<String> holding "bed-4 free", in Arc<RwLock<_>>.
    // Spawn one charge-nurse thread that pushes "bed-9 free" through .write(), and join
    // it. Then spawn 3 nurse threads that each read the board's length through .read().
    // Join them and return the count the nurses see, which should be 2.
    0
}

fn main() {
    println!("[board] free beds visible: {} (want 2)", free_beds_seen());
    println!("══ every nurse sees the same board — Night Shift is complete ══");
}
