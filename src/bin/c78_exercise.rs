// BUG: The "busiest 3-hour window" report always misses the closing rush. On a day where the
// last three hours were the busiest, the report points at some earlier, quieter window. It's
// as if the very last window never gets checked. The code compiles and runs. Find and fix it.
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
    let hourly = [4500, 3500, 6000, 12000, 9000];
    println!("busiest 3-hour window: {} cents", busiest_window(&hourly, 3));
}
