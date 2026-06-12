// Warmup (no new Rust concepts): detect a duplicate by inserting into a HashSet — insert returns
// false if the value was already present. Coming from C: a hash-set membership check, no nested loops.
use std::collections::HashSet;

fn has_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    nums.iter().any(|&n| !seen.insert(n))
}

fn main() {
    println!("{}", has_duplicate(&[1, 2, 3, 1])); // true
    println!("{}", has_duplicate(&[1, 2, 3])); // false
}
