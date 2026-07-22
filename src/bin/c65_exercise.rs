// THE VAULT RUN — Chapter 4: THE CREW
// Forged access signatures are single-use — a replay trips the alarm daemon.
// Scan the batch in one pass. (Warmup: no new Rust concepts.)
#[allow(unused_imports)]
use std::collections::HashSet;

pub fn signature_reused(sigs: &[i32]) -> bool {
    // TODO: Return true if any signature appears more than once.
    // A HashSet is a HashMap with keys only — `insert` returns false when the
    // value was already present, which is exactly the replay signal.
    let _ = sigs;
    false
}

fn main() {
    println!("[alarm] reused? {} (want true)", signature_reused(&[7011, 7012, 7013, 7011]));
    println!("[alarm] reused? {} (want false)", signature_reused(&[7011, 7012, 7013]));
}
