pub fn debug_string(items: &[i32]) -> String {
    // TODO: Return the Debug format of `items` as a String.
    // Hint: format!("{:?}", ...)
    format!("{:?}", items)
}

fn main() {
    println!("{}", debug_string(&[1, 2, 3]));
}
