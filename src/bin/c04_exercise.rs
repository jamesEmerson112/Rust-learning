pub fn min_max(values: [i32; 5]) -> (i32, i32) {
    // TODO: Return a tuple of (min, max) from the array.
    let min: i32 = *values.iter().min().expect("something is here");
    let max: i32 = *values.iter().max().expect("something is here");
    (min, max)
}

fn main() {
    let values = [3, 1, 4, 1, 5];
    let (min, max) = min_max(values);
    println!("Min: {min}, Max: {max}");
}
