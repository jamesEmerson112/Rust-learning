#[allow(unused_imports)]
use std::collections::HashSet;

pub fn has_duplicate(nums: &[i32]) -> bool {
    // TODO: Return true if any value appears more than once.
    // A HashSet is a HashMap with keys only — `insert` returns false when the
    // value was already present, which is exactly the duplicate signal.
    let _ = nums;
    false
}

fn main() {
    println!("{}", has_duplicate(&[1, 2, 3, 1]));
    println!("{}", has_duplicate(&[1, 2, 3]));
}
