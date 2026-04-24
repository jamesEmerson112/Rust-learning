pub fn product(nums: &[i32]) -> i32 {
    // TODO: Return the product of all items in `nums`.
    // Hint: nums.iter().fold(initial, |acc, &n| ...)
    nums.iter().fold(1, |acc, &n| acc * n )
}

fn main() {
    println!("{}", product(&[1, 2, 3, 4]));
}
