fn sum_of_squares(nums: &[i32]) -> i32 {
    nums.iter().map(|&n| n * n).sum()
}

fn main() {
    let data = [1, 2, 3, 4];
    println!("sum = {}", data.iter().sum::<i32>());
    println!("max = {:?}", data.iter().max());
    println!("sum_of_squares = {}", sum_of_squares(&data));
}
