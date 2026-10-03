// Warmup (no new Rust concepts): reverse a Vec in place by swapping its two ends and moving
// inward. This is the two-pointer technique, where one index starts at each end and the two
// indices meet in the middle.
// Coming from C: this is the classic swap loop with indices i and j. Here v.swap(i, j) does
// the swap without any unsafe code.
//
// RUST GENERAL HOSPITAL: the delivery robot records every room it passes. To drive back to
// the pharmacy, it retraces that list backwards. The list is reversed in place, with no new
// allocation.
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

    // The idiomatic alternative is shorter, but it allocates a new Vec:
    let copied: Vec<i32> = vec![1, 2, 3].iter().rev().copied().collect();
    println!("[robot] allocating variant: {copied:?}");
}
