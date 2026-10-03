// Bug Hunt drill: slice window boundaries. To scan every run of `width` consecutive items in
// a slice of length n, note that the last valid start index is n - width. The loop range
// must therefore include its end: 0..=(n - width). With `..` instead, the loop skips the
// final window, which is a classic off-by-one error.
// Coming from C: this is `for (start = 0; start <= n - width; start++)`. Everything depends
// on writing `<=` rather than `<`. Rust writes the same distinction as `..=` versus `..`.
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
    // ER arrivals per hour. The busiest 3-hour window is the late rush at the end.
    let hourly = [4, 3, 6, 12, 9];
    println!("busiest 3-hour window: {} arrivals", busiest_window(&hourly, 3));
}
