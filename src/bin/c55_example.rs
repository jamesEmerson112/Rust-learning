// Warmup (no new Rust concepts): iterative Fibonacci with a rolling pair — limbering up
// before the run on Aegis-9. Coming from C: a plain for-loop with two accumulators,
// nothing borrowed.
//
// THE VAULT RUN — Chapter 1: LOADOUT. The vault's rolling access code is Fibonacci-derived.
// Old crypto, corporate arrogance. Crack the keygen and we're in business.
fn access_code(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    println!("[CHROME SURGEON] jacking in... warming up the keygen");
    let stream: Vec<u64> = (0..10).map(access_code).collect();
    println!("[keygen] keystream sample: {stream:?}");
    println!("[keygen] access_code(50) = {} — the vault rotates deep", access_code(50));
}
