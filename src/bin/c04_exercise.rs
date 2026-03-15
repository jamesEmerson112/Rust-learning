pub fn min_max(values: [i32; 5]) -> (i32, i32) {
    // TODO: Return a tuple of (min, max) from the array.
    let _ = values;
    (0, 0)
}

fn main() {
    let values = [3, 1, 4, 1, 5];
    let (min, max) = min_max(values);
    println!("Min: {min}, Max: {max}");
}
