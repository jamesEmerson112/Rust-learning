// BUG: The emergency room's "busiest 3-hour window" report always misses the late-night rush.
// On a night where the last three hours were the busiest, the report picks an earlier,
// quieter window, so the staff rota never adds people where they are needed. The report
// behaves as if the last window is never checked. The code compiles and runs. Find the bug
// and fix it.
// This drills c08 and c39 to c41. The tests in tests/c78_tests.rs must pass.
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
