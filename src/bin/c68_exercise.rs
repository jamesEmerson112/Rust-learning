// RUST GENERAL HOSPITAL — Night Shift
// The bed board: three nurses read it constantly, the charge nurse writes it.
// RwLock = many readers OR one writer — reads don't queue behind each other.
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};
#[allow(unused_imports)]
use std::thread;

pub fn free_beds_seen() -> usize {
    // TODO: Wrap a bed board (Vec<String>) in Arc<RwLock<_>> seeded with
    // "bed-4 free". Spawn ONE charge-nurse thread that .write()s "bed-9 free"
    // and join it, then spawn 3 nurse threads that .read() the board length.
    // Join them and return the count the nurses see (want 2).
    0
}

fn main() {
    println!("[board] free beds visible: {} (want 2)", free_beds_seen());
    println!("══ every nurse sees the same board — Night Shift is complete ══");
}
