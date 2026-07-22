// Warmup (no new Rust concepts): reverse a Vec in place by swapping ends inward (two-pointer).
// Coming from C: the classic for(i,j) swap loop — here v.swap(i, j) does it without unsafe.
//
// THE VAULT RUN — Chapter 3: INSIDE THE ICE. Aegis-9's inner ICE reads packets tail-first.
// Reverse each buffer in place before retransmit — no allocations, we're running hot.
fn reverse_packet(buf: &mut Vec<i32>) {
    let n = buf.len();
    for i in 0..n / 2 {
        buf.swap(i, n - 1 - i);
    }
}

fn main() {
    let mut packet = vec![0x1F, 0x2A, 0x33, 0x4C, 0x55];
    println!("[ice] outbound: {packet:?}");
    reverse_packet(&mut packet);
    println!("[ice] retransmit (tail-first): {packet:?}");

    // Idiomatic alternative — clean, but allocates a new Vec (too slow in the ice):
    let copied: Vec<i32> = vec![1, 2, 3].iter().rev().copied().collect();
    println!("[ice] allocating variant: {copied:?}");
}
