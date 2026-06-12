// Warmup (no new Rust concepts): reverse a Vec in place by swapping ends inward (two-pointer).
// Coming from C: the classic for(i,j) swap loop — here v.swap(i, j) does it without unsafe.
fn reverse_in_place(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 0..n / 2 {
        v.swap(i, n - 1 - i);
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut v);
    println!("{v:?}");

    // Idiomatic alternative — clean, but allocates a new Vec:
    let reversed: Vec<i32> = vec![1, 2, 3].iter().rev().copied().collect();
    println!("{reversed:?}");
}
