// Warmup (no new Rust concepts): Two Sum in one pass — remember each number's index in a
// HashMap, then look for its complement. Coming from C: a hash table replacing the O(n^2)
// double loop.
//
// THE VAULT RUN — Chapter 2: GHOST PROTOCOL. You intercepted a burst of numeric codes.
// Intel says exactly two of them sum to the master key. Find the pair, keep the indices.
use std::collections::HashMap;

fn master_key_pair(codes: &[i32], master: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &code) in codes.iter().enumerate() {
        if let Some(&j) = seen.get(&(master - code)) {
            return Some((j, i));
        }
        seen.insert(code, i);
    }
    None
}

fn main() {
    let intercepted = [2, 7, 11, 15];
    println!("[intercept] codes: {intercepted:?}, master key: 9");
    println!("[intercept] pair: {:?}", master_key_pair(&intercepted, 9));
}
