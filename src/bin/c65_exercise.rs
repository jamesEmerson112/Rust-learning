// RUST GENERAL HOSPITAL — Night Shift
// Each medication order the delivery robot carries must arrive exactly once — a repeat
// means a possible double dose. Check the delivery log in one pass.
// (Warmup: no new Rust concepts.)
#[allow(unused_imports)]
use std::collections::HashSet;

pub fn double_delivery(order_ids: &[i32]) -> bool {
    // TODO: Return true if any order ID appears more than once.
    // A HashSet is a HashMap with keys only — `insert` returns false when the
    // value was already present, which is exactly the double-delivery signal.
    let _ = order_ids;
    false
}

fn main() {
    println!("[robot] double delivery? {} (want true)", double_delivery(&[7011, 7012, 7013, 7011]));
    println!("[robot] double delivery? {} (want false)", double_delivery(&[7011, 7012, 7013]));
}
