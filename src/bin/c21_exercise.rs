pub fn doubled(nums: &[i32]) -> Vec<i32> {
    // TODO: Return a Vec where every element of `nums` is doubled.
    // Hint: nums.iter().map(|&n| ...).collect()
    nums.iter().map(|&n| n*2).collect()
}

fn main() {
    println!("{:?}", doubled(&[1, 2, 3, 4]));
}
