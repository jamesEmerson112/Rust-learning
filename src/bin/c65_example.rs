// Warmup (no new Rust concepts): detect a duplicate by inserting each value into a HashSet.
// insert returns false if the value was already present.
// Coming from C: this is a hash-set membership check, which replaces a pair of nested loops.
//
// RUST GENERAL HOSPITAL: every medication order the delivery robot carries must arrive exactly
// once. If one is delivered twice, a patient could get a double dose. The code checks the
// delivery log in one pass.
use std::collections::HashSet;

fn double_delivery(order_ids: &[i32]) -> bool {
    let mut seen = HashSet::new();
    order_ids.iter().any(|&id| !seen.insert(id))
}

fn main() {
    println!("[robot] log A has a double delivery? {}", double_delivery(&[7011, 7012, 7013, 7011])); // true
    println!("[robot] log B has a double delivery? {}", double_delivery(&[7011, 7012, 7013])); // false
}
