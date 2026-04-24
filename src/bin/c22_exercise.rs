pub fn squared_evens(nums: &[i32]) -> Vec<i32> {
    // TODO: Return the squares of even numbers only.
    // .filter(|&&n| ...) — two &'s because .iter() yields &i32,
    // and .filter() borrows that, giving &&i32 to the closure.
    // Then .map(|&n| ...).collect() as you learned in c21.
    nums.iter().filter(|&n| n % 2 == 0).map(|n| n * n).collect()
}

fn main() {
    let data = [1, 2, 3, 4, 5, 6];

    // 1) .iter() → yields &i32
    let iter_step: Vec<&i32> = data.iter().collect();
    println!("iter:    {:?}", iter_step);

    // 2) .filter(|&&n| n % 2 == 0) → keeps only even, still &i32
    let filter_step: Vec<&i32> = data.iter().filter(|&&n| n % 2 == 0).collect();
    println!("filter:  {:?}", filter_step);

    // 3) .map(|&n| n * n) → transforms &i32 into i32
    let map_step: Vec<i32> = data.iter().filter(|&&n| n % 2 == 0).map(|&n| n * n).collect();
    println!("map:     {:?}", map_step);

    // 4) .collect() → same as above, that IS the collect
    println!("collect: {:?}", squared_evens(&data));
}
