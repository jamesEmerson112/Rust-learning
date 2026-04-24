fn product(nums: &[i32]) -> i32 {
    nums.iter().fold(1, |acc, &n| acc * n)
}

fn main() {
    println!("product = {}", product(&[1, 2, 3, 4]));
}
