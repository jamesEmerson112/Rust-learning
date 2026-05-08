use std::time::Instant;

fn main() {
    let nums: Vec<i32> = (1..=10_000_000).collect();

    let start = Instant::now();
    let total: i64 = nums.iter().map(|&n| n as i64).sum();
    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);
    eprintln!("sum = {total}");
}
