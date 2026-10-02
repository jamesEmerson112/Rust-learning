// RUST GENERAL HOSPITAL — Night Shift
// One supply list, three nurses, three threads. Each nurse double-checks the count
// independently. Share the list — don't copy it.
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::thread;

pub fn supply_check() -> u32 {
    // TODO: Wrap the supply list vec![4000u32, 6500, 3500] in an Arc. Spawn 3
    // threads that each Arc::clone a handle and sum the list, then join all
    // three and return the grand total of their counts (3 × 14000 = 42000).
    // (Plain Rc would NOT compile here — it isn't Send across threads; Arc is.)
    0
}

fn main() {
    println!("[ward] all three counts together: {} (want 42000)", supply_check());
}
