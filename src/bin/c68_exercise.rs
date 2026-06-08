#[allow(unused_imports)]
use std::sync::{Arc, RwLock};
#[allow(unused_imports)]
use std::thread;

pub fn read_write_schedule() -> usize {
    // TODO: Wrap a schedule (Vec<String>) in Arc<RwLock<_>> seeded with one
    // appointment. Spawn ONE writer thread that .write()s a second appointment
    // and join it, then spawn 3 reader threads that .read() the length. Return
    // the count the readers see. RwLock = many readers OR one writer at a time.
    0
}

fn main() {
    println!("appointments: {}", read_write_schedule());
}
