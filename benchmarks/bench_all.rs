use std::collections::HashMap;
use std::time::Instant;

fn double(x: i32) -> i32 { x * 2 }
fn sum_array(values: &[i32]) -> i64 { values.iter().map(|&n| n as i64).sum() }
fn evens_up_to(n: u32) -> Vec<u32> { (1..=n).filter(|i| i % 2 == 0).collect() }
fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut m = HashMap::new();
    for c in s.chars() { *m.entry(c).or_insert(0) += 1; }
    m
}
fn doubled(nums: &[i32]) -> Vec<i32> { nums.iter().map(|&n| n * 2).collect() }
fn squared_evens(nums: &[i32]) -> Vec<i64> {
    nums.iter().filter(|&&n| n % 2 == 0).map(|&n| (n as i64) * (n as i64)).collect()
}
fn total(nums: &[i32]) -> i64 { nums.iter().map(|&n| n as i64).sum() }
fn product(nums: &[i32]) -> i64 { nums.iter().fold(1i64, |acc, &n| acc * (n as i64)) }
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in text.split_whitespace() {
        *m.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    m
}

fn bench<F: FnOnce()>(label: &str, f: F) {
    let start = Instant::now();
    f();
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("{:<28} | {:>10.1} ms", label, ms);
}

fn main() {
    let data10m: Vec<i32> = (0..10_000_000).collect();
    let data5m: Vec<i32> = (0..5_000_000).collect();
    let ones1m: Vec<i32> = vec![1; 1_000_000];
    let chartext: String = "abcde".repeat(2_000_000);
    let wordtext: String = std::iter::repeat("hello world rust python")
        .take(250_000)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{:<28} | {:>12}", "Task", "Rust (ms)");
    println!("{}", "-".repeat(45));

    // c01 double — 10M calls
    bench("c01 double (10M calls)", || {
        for i in 0..10_000_000i32 { std::hint::black_box(double(i)); }
    });

    // c03 sum_array — 10M
    bench("c03 sum_array (10M)", || { std::hint::black_box(sum_array(&data10m)); });

    // c05 evens_up_to — 5M
    bench("c05 evens_up_to (5M)", || { std::hint::black_box(evens_up_to(5_000_000)); });

    // c13 count_chars — 10M
    bench("c13 count_chars (10M)", || { std::hint::black_box(count_chars(&chartext)); });

    // c21 doubled — 5M
    bench("c21 doubled (5M)", || { std::hint::black_box(doubled(&data5m)); });

    // c22 squared_evens — 5M
    bench("c22 squared_evens (5M)", || { std::hint::black_box(squared_evens(&data5m)); });

    // c23 total — 10M
    bench("c23 total (10M)", || { std::hint::black_box(total(&data10m)); });

    // c24 product — 1M
    bench("c24 product (1M)", || { std::hint::black_box(product(&ones1m)); });

    // c16 word_count — 1M words
    bench("c16 word_count (1M words)", || { std::hint::black_box(word_count(&wordtext)); });
}
