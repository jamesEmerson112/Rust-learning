// RUST GENERAL HOSPITAL — Safe Shutdown
// The delivery robot has a little battery left before it must go back to its dock.
// Each number in `legs` is the battery cost of one hallway. Find two hallways whose
// costs add up to exactly the remaining `charge`, and return their positions. ONE pass.
// (Warmup: no new Rust concepts.)
#[allow(unused_imports)]
use std::collections::HashMap;

pub fn trip_pair(legs: &[i32], charge: i32) -> Option<(usize, usize)> {
    // TODO: Return indices (i, j) with i < j where legs[i] + legs[j] == charge,
    // or None. One pass with a HashMap: for each leg, check whether its
    // complement (charge - leg) has already been seen.
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &leg) in legs.iter().enumerate() {
        if let Some(&j) = seen.get(&(charge-leg)) {
            return Some((j, i));
        }
        seen.insert(leg, i);
    }
    None
}

fn main() {
    println!("[robot] pair: {:?} (want Some((0, 1)))", trip_pair(&[2, 7, 11, 15], 9));
}
