// Warmup (no new Rust concepts): Two Sum in one pass — remember each number's index in a
// HashMap, then look for its complement. Coming from C: a hash table replacing the O(n^2)
// double loop.
//
// RUST GENERAL HOSPITAL: the delivery robot has 9 units of battery left. Each number in
// `legs` is the battery cost of one hallway. Find two hallways whose costs add up to
// exactly 9, and return their positions in the list.
use std::collections::HashMap;

fn trip_pair(legs: &[i32], charge: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &leg) in legs.iter().enumerate() {
        if let Some(&j) = seen.get(&(charge - leg)) {
            return Some((j, i));
        }
        seen.insert(leg, i);
    }
    None
}

fn main() {
    let legs = [2, 7, 11, 15];
    println!("[robot] hallway legs: {legs:?}, charge left: 9");
    println!("[robot] pair: {:?}", trip_pair(&legs, 9));
}
