// Bug Hunt drill: slice-window boundaries. To scan every contiguous window of `width` over a
// slice of length n, the last valid start is n - width, so the loop must be INCLUSIVE:
// 0..=(n - width). Use `..` and you drop the final window — a textbook off-by-one.
// Coming from C: `for (start = 0; start <= n - width; start++)` — the `<=` (not `<`) is the
// whole ballgame. Rust spells it `..=` vs `..`.
pub fn busiest_window(hourly: &[u32], width: usize) -> u32 {
    if width == 0 || width > hourly.len() {
        return 0;
    }
    let mut best = 0;
    for start in 0..=(hourly.len() - width) {
        let sum: u32 = hourly[start..start + width].iter().sum();
        if sum > best {
            best = sum;
        }
    }
    best
}

fn main() {
    // Hourly revenue in cents; the busiest 3-hour window is the closing rush at the end.
    let hourly = [4500, 3500, 6000, 12000, 9000];
    println!("busiest 3-hour window: {} cents", busiest_window(&hourly, 3));
}
