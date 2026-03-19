fn sum_array(values: [i32; 4]) -> i32 {
    values.iter().sum()
}

fn main() {
    let mut values: [i32; 4] = [10, 20, 30, 40];
    println!("Array: {values:?}");
    println!("Sum: {}", sum_array(values));

    // Iterating element by element
    for v in values.iter_mut() {
        print!("{v} ");
        *v += 1;
    }
    
    println!();

    let my_strings = [format!("Hello"), format!("James!"), String::new(), String::default()];

    my_func(my_strings);
}

fn my_func(array: [String; 4]) {
    for string in array.iter() {
        println!("{string}");
    }
}