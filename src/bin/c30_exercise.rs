pub fn boxed_price(cents: i32) -> Box<i32> {
    // TODO: Return `cents` inside a Box (heap-allocated).
    // Hint: Box::new(...)
    let _ = cents;
    Box::new(0)
}

fn main() {
    let price = boxed_price(4500);
    println!("Gel Manicure: {} cents", *price);
}
