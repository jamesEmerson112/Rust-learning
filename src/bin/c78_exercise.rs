// BUG: The emergency room's "busiest 3-hour window" report always misses the late-night rush.
// On a night where the last three hours were the busiest, the report points at some earlier,
// quieter window, so the rota never adds staff where they're needed. It's as if the very last
// window never gets checked. The code compiles and runs. Find and fix it.
// (This drills c08/c39-c41: slices + range bounds. The tests in tests/c78_tests.rs must go green.)
pub fn busiest_window(hourly: &[u32], width: usize) -> u32 {
    if width == 0 || width > hourly.len() {
        return 0;
    }
    let mut best = 0;
    for start in 0..(hourly.len() - width) {
        let sum: u32 = hourly[start..start + width].iter().sum();
        if sum > best {
            best = sum;
        }
    }
    best
}

fn main() {
    let hourly = [4, 3, 6, 12, 9];
    println!("busiest 3-hour window: {} arrivals", busiest_window(&hourly, 3));
}
