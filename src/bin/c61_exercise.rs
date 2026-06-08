#[allow(unused_imports)]
use std::rc::{Rc, Weak};

pub fn weak_counts() -> (usize, usize) {
    // TODO: Create an Rc, downgrade it once to a Weak, then return
    // (strong_count, weak_count). A Weak does NOT raise the strong count.
    (0, 0)
}

pub fn dangling_weak_is_none() -> bool {
    // TODO: Downgrade an Rc to a Weak, drop the Rc, then show the Weak can no
    // longer be upgraded. This is how back-references avoid keeping data alive
    // (and how Rc cycles avoid leaking — make one direction Weak).
    false
}

fn main() {
    println!("counts = {:?}", weak_counts());
    println!("dangling weak is none = {}", dangling_weak_is_none());
}
