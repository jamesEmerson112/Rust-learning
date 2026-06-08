#[allow(unused_imports)]
use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // TODO: Return indices (i, j) with i < j where nums[i] + nums[j] == target,
    // or None. One pass with a HashMap: for each value, check whether its
    // complement (target - value) has already been seen.
    let _ = (nums, target);
    None
}

fn main() {
    println!("{:?}", two_sum(&[2, 7, 11, 15], 9));
}
