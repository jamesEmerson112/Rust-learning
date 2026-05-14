fn boxed_price(cents: i32) -> Box<i32> {
    Box::new(cents)
}

fn main() {
    let price = boxed_price(4500);
    println!("Gel Manicure: {} cents", *price);
}
