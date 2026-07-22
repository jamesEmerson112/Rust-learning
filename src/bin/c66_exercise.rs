// THE VAULT RUN — Chapter 4: THE CREW
// One vault map, three crew members, three threads. Share it — don't copy it.
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use std::thread;

pub fn crew_estimates() -> u32 {
    // TODO: Wrap the vault map vec![4000u32, 6500, 3500] in an Arc. Spawn 3
    // threads that each Arc::clone a handle and sum the map, then join all
    // three and return the grand total of their estimates (3 × 14000 = 42000).
    // (Plain Rc would NOT compile here — it isn't Send across threads; Arc is.)
    0
}

fn main() {
    println!("[crew] combined estimates: {} (want 42000)", crew_estimates());
}
