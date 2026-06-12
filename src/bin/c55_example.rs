// Warmup (no new Rust concepts): iterative Fibonacci with a rolling pair — a breather before
// the smart-pointer block. Coming from C: a plain for-loop with two accumulators, nothing borrowed.
fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    let seq: Vec<u64> = (0..10).map(fib).collect();
    println!("First 10 Fibonacci numbers: {seq:?}");
}
