fn boxed_number(n: i32) -> Box<i32> {
    Box::new(n)
}

fn main() {
    let b = boxed_number(42);
    println!("boxed = {b}");
    println!("deref = {}", *b);
}
