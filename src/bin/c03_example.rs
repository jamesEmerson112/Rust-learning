fn sum_array(values: [i32; 4]) -> i32 {
    values.iter().sum()
}

fn main() {
    let values: [i32; 4] = [10, 20, 30, 40];
    println!("Array: {values:?}");
    println!("Sum: {}", sum_array(values));

    // Iterating element by element
    for v in values.iter() {
        print!("{v} ");
    }
    println!();
}
