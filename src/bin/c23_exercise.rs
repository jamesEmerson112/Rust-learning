pub fn total(nums: &[i32]) -> i32 {
    // TODO: Return the sum of all items in `nums`.
    // Hint: nums.iter().sum()
    nums.iter().sum()
}

fn main() {
    println!("{}", total(&[1, 2, 3, 4]));
}
