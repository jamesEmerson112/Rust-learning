pub fn boxed_number(n: i32) -> Box<i32> {
    // TODO: Return `n` inside a Box (heap-allocated).
    // Hint: Box::new(...)
    let _ = n;
    Box::new(0)
}

fn main() {
    let b = boxed_number(42);
    println!("boxed = {b}");
}
