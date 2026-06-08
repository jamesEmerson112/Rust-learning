use std::collections::HashSet;

fn has_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    nums.iter().any(|&n| !seen.insert(n))
}

fn main() {
    println!("{}", has_duplicate(&[1, 2, 3, 1])); // true
    println!("{}", has_duplicate(&[1, 2, 3])); // false
}
