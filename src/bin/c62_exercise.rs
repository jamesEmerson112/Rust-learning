// RUST GENERAL HOSPITAL — Shared Care
// The delivery robot drives back to the pharmacy by retracing the rooms it passed.
// Reverse its path in place — the robot's controller has no memory to spare for a
// second Vec. (Warmup: no new Rust concepts.)

pub fn retrace(path: &mut Vec<i32>) {
    // TODO: Reverse `path` in place — no allocation. Walk two pointers inward and
    // swap position i with position n-1-i for the first half.
    // (Idiomatic alternative: .iter().rev().collect(), but that allocates.)
    let _ = path;
}

fn main() {
    let mut path = vec![101, 104, 210, 215, 302];
    retrace(&mut path);
    println!("[robot] path back: {path:?} (want [302, 215, 210, 104, 101])");
}
