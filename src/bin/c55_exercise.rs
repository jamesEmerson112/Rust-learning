// RUST GENERAL HOSPITAL — Ward Equipment
// The medicine-delivery robot climbs stairs one or two steps at a time. Count how many
// different step patterns get it up a staircase. (Warmup: no new Rust concepts.)

pub fn climb_ways(steps: u32) -> u64 {
    // TODO: Return the number of ways to climb `steps` steps, taking 1 or 2 at a time.
    // climb_ways(0) = 1 (already at the top), climb_ways(1) = 1, climb_ways(2) = 2.
    // Hint: keep a rolling pair and swap `steps` times: (a, b) = (b, a + b).
    let (mut a, mut b) = (0, 0);

    for _ in 0..steps {
        (a, b) = (b, a+b);
    }

    a
}

pub fn ways_table(len: usize) -> Vec<u64> {
    // TODO: Return climb_ways for 0, 1, 2, ... up to len - 1, in order.
    // Hint: (0..len).map(|n| climb_ways(n as u32)).collect()
    (0..len).map(|n| climb_ways(n as u32)).collect()
}

fn main() {
    println!("[robot] climb_ways(10) = {} (want 89)", climb_ways(10));
    println!("[robot] ways_table(8) = {:?}", ways_table(8));
}
