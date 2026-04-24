fn total(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    println!("total = {}", total(&[1, 2, 3, 4]));
}
