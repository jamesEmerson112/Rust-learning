// Warmup (no new Rust concepts): iterative Fibonacci with a rolling pair. Coming from C:
// a plain for-loop with two accumulators, nothing borrowed.
//
// RUST GENERAL HOSPITAL: the medicine-delivery robot climbs stairs one or two steps at a
// time. The number of different step patterns that get it up n steps is a Fibonacci number.
fn climb_ways(steps: u32) -> u64 {
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 0..steps {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    let table: Vec<u64> = (0..10).map(climb_ways).collect();
    println!("[robot] ways to climb 0..10 steps: {table:?}");
    println!("[robot] a 50-step stairwell: {} step patterns", climb_ways(50));
}
