fn doubled(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|&n| n * 2).collect()
}

fn main() {
    let data = [1, 2, 3, 4];
    println!("{:?}", doubled(&data));
}
