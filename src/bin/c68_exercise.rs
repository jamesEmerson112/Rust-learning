// THE VAULT RUN — Chapter 4: THE CREW
// The alert board: three lookouts read it constantly, one spotter writes.
// RwLock = many readers OR one writer — reads don't queue behind each other.
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};
#[allow(unused_imports)]
use std::thread;

pub fn alert_board_count() -> usize {
    // TODO: Wrap an alert board (Vec<String>) in Arc<RwLock<_>> seeded with
    // "all-quiet". Spawn ONE spotter thread that .write()s "guards-rotating"
    // and join it, then spawn 3 lookout threads that .read() the board length.
    // Join them and return the count the lookouts see (want 2).
    0
}

fn main() {
    println!("[board] alerts visible: {} (want 2)", alert_board_count());
    println!("══ the crew is in position — CHAPTER 4: THE CREW is complete ══");
}
