pub fn boxed_price(cents: i32) -> Box<i32> {
    // TODO: Return `cents` inside a Box (heap-allocated).
    // Hint: Box::new(...)
    Box::new(cents)
}

fn main() {
    let price = boxed_price(4500);
    println!("Gel Manicure: {} cents", *price);
}
