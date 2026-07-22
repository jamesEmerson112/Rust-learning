// THE VAULT RUN — Chapter 3: INSIDE THE ICE
// The inner ICE reads packets tail-first. Reverse the buffer in place before
// retransmit — no new Vec, we're running hot. (Warmup: no new Rust concepts.)

pub fn reverse_packet(buf: &mut Vec<i32>) {
    // TODO: Reverse `buf` in place — no allocation. Walk two pointers inward and
    // swap position i with position n-1-i for the first half.
    // (Idiomatic alternative: .iter().rev().collect(), but that allocates.)
    let _ = buf;
}

fn main() {
    let mut packet = vec![0x1F, 0x2A, 0x33, 0x4C, 0x55];
    reverse_packet(&mut packet);
    println!("[ice] retransmit: {packet:?} (want [85, 76, 51, 42, 31])");
}
