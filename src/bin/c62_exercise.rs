// RUST GENERAL HOSPITAL — Shared Care
// The delivery robot drives back to the pharmacy by retracing the rooms it passed.
// Reverse its path in place, because the robot's controller has no memory to spare for a
// second Vec. (Warmup: no new Rust concepts.)

pub fn retrace(path: &mut Vec<i32>) {
    // TODO: Reverse `path` in place without allocating. Walk two indices inward from both
    // ends, and for each position i in the first half, swap it with position n-1-i.
    // The idiomatic alternative is .iter().rev().collect(), but that allocates a new Vec.
    let _ = path;
}

fn main() {
    let mut path = vec![101, 104, 210, 215, 302];
    retrace(&mut path);
    println!("[robot] path back: {path:?} (want [302, 215, 210, 104, 101])");
}
