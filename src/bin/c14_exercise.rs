use std::collections::HashMap;

pub fn count_numbers(nums: &[i32]) -> HashMap<i32, usize> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let nums = [1, 2, 2, 3, 3, 3];
    let counts = count_numbers(&nums);
    println!("{counts:?}");
}
