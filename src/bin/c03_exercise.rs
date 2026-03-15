pub fn sum_array(values: [i32; 4]) -> i32 {
    // TODO: Return the sum of all elements in the array.
    let _ = values;
    values.iter().sum()
}

fn main() {
    let values = [1, 2, 3, 4];
    println!("Sum: {}", sum_array(values));
}
