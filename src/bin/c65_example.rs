// Warmup (no new Rust concepts): detect a duplicate by inserting into a HashSet — insert returns
// false if the value was already present. Coming from C: a hash-set membership check, no nested loops.
//
// RUST GENERAL HOSPITAL: every medication order the delivery robot carries must arrive exactly
// once. Deliver one twice and a patient could get a double dose. Check the log in one pass.
use std::collections::HashSet;

fn double_delivery(order_ids: &[i32]) -> bool {
    let mut seen = HashSet::new();
    order_ids.iter().any(|&id| !seen.insert(id))
}

fn main() {
    println!("[robot] log A has a double delivery? {}", double_delivery(&[7011, 7012, 7013, 7011])); // true
    println!("[robot] log B has a double delivery? {}", double_delivery(&[7011, 7012, 7013])); // false
}
