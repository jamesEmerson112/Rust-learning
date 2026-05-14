#[allow(unused_imports)]
use std::rc::Rc;

pub fn shared_stations(price: i32) -> usize {
    // TODO: Create an Rc holding `price`, clone it twice (3 stations total),
    // and return Rc::strong_count (should be 3).
    let _ = price;
    0
}

fn main() {
    println!("stations sharing price list: {}", shared_stations(4500));
}
