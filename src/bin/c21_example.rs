fn squared_evens(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect()
}

fn main() {
    let input = [1, 2, 3, 4, 5, 6];
    println!("squared_evens({input:?}) = {:?}", squared_evens(&input));
}
