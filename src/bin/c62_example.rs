// Warmup (no new Rust concepts): reverse a Vec in place by swapping ends inward (two-pointer).
// Coming from C: the classic for(i,j) swap loop — here v.swap(i, j) does it without unsafe.
//
// RUST GENERAL HOSPITAL: the delivery robot records every room it passes. To drive back to
// the pharmacy it retraces that list backwards, reversed in place, with no new allocation.
fn retrace(path: &mut Vec<i32>) {
    let n = path.len();
    for i in 0..n / 2 {
        path.swap(i, n - 1 - i);
    }
}

fn main() {
    let mut path = vec![101, 104, 210, 215, 302]; // rooms passed on the way out
    println!("[robot] path out: {path:?}");
    retrace(&mut path);
    println!("[robot] path back: {path:?}");

    // Idiomatic alternative — clean, but allocates a new Vec:
    let copied: Vec<i32> = vec![1, 2, 3].iter().rev().copied().collect();
    println!("[robot] allocating variant: {copied:?}");
}
