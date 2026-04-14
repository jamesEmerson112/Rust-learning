use std::collections::HashMap;

fn count_numbers(nums: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for &n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let nums = [1, 2, 2, 3, 3, 3, 4];
    let counts = count_numbers(&nums);
    for (n, c) in &counts {
        println!("{n}: {c}");
    }
}
