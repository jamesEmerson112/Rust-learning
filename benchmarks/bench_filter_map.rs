use std::time::Instant;

fn main() {
    let nums: Vec<i32> = (1..=10_000_000).collect();

    let start = Instant::now();
    let result: Vec<i64> = nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| (n as i64) * (n as i64))
        .collect();
    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);
    eprintln!("len = {}", result.len());
}
