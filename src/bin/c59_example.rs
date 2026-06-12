// Warmup (no new Rust concepts): Two Sum in one pass — remember each number's index in a
// HashMap, then look for its complement. Coming from C: a hash table replacing the O(n^2) double loop.
use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return Some((j, i));
        }
        seen.insert(n, i);
    }
    None
}

fn main() {
    println!("{:?}", two_sum(&[2, 7, 11, 15], 9));
}
