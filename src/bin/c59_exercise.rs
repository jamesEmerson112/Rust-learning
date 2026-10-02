// RUST GENERAL HOSPITAL — Safe Shutdown
// The delivery robot has just enough charge for one last two-leg trip before it must dock.
// Exactly two hallway legs use up the remaining charge. Find them in ONE pass.
// (Warmup: no new Rust concepts.)
#[allow(unused_imports)]
use std::collections::HashMap;

pub fn trip_pair(legs: &[i32], charge: i32) -> Option<(usize, usize)> {
    // TODO: Return indices (i, j) with i < j where legs[i] + legs[j] == charge,
    // or None. One pass with a HashMap: for each leg, check whether its
    // complement (charge - leg) has already been seen.
    let _ = (legs, charge);
    None
}

fn main() {
    println!("[robot] pair: {:?} (want Some((0, 1)))", trip_pair(&[2, 7, 11, 15], 9));
}
