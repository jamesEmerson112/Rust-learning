// THE VAULT RUN — Chapter 2: GHOST PROTOCOL
// You intercepted a burst of numeric codes off the Aegis-9 maintenance band. Exactly two
// of them sum to the master key. Find them in ONE pass. (Warmup: no new Rust concepts.)
#[allow(unused_imports)]
use std::collections::HashMap;

pub fn master_key_pair(codes: &[i32], master: i32) -> Option<(usize, usize)> {
    // TODO: Return indices (i, j) with i < j where codes[i] + codes[j] == master,
    // or None. One pass with a HashMap: for each code, check whether its
    // complement (master - code) has already been seen.
    let _ = (codes, master);
    None
}

fn main() {
    println!("[intercept] pair: {:?} (want Some((0, 1)))", master_key_pair(&[2, 7, 11, 15], 9));
}
