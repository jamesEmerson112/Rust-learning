pub fn reverse_in_place(v: &mut Vec<i32>) {
    // TODO: Reverse `v` in place — no new Vec. Walk two pointers inward and
    // swap position i with position n-1-i for the first half.
    // (Idiomatic alternative: v.iter().rev().collect(), but that allocates.)
    let _ = v;
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut v);
    println!("{v:?}");
}
