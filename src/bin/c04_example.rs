fn min_max(values: [i32; 5]) -> (i32, i32) {
    let min = *values.iter().min().expect("array is non-empty");
    let max = *values.iter().max().expect("array is non-empty");
    (min, max)
}

fn main() {
    let scores = [72, 85, 90, 63, 91];
    let (lo, hi) = min_max(scores);
    println!("Min: {lo}, Max: {hi}");

    // Type casting with `as`
    let sum: i32 = scores.iter().sum();
    let avg = sum as f64 / scores.len() as f64;
    println!("Average: {avg:.2}");
}
