// Warmup (no new Rust concepts): detect a duplicate by inserting into a HashSet — insert returns
// false if the value was already present. Coming from C: a hash-set membership check, no nested loops.
//
// THE VAULT RUN — Chapter 4: THE CREW. Every forged access signature must be single-use.
// Replay one and Aegis-9's alarm daemon lights up. Scan the batch before the crew goes in.
use std::collections::HashSet;

fn signature_reused(sigs: &[i32]) -> bool {
    let mut seen = HashSet::new();
    sigs.iter().any(|&s| !seen.insert(s))
}

fn main() {
    println!("[alarm] batch A reused? {}", signature_reused(&[7011, 7012, 7013, 7011])); // true
    println!("[alarm] batch B reused? {}", signature_reused(&[7011, 7012, 7013])); // false
}
