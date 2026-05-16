#[allow(unused_imports)]
use std::rc::Rc;

pub fn shared_stations(price: i32) -> usize {
    // TODO: Create an Rc holding `price`, clone it twice (3 stations total),
    // and return Rc::strong_count (should be 3).
    let price = Rc::new(price);
    let _station_1 = Rc::clone(&price);
    let _station_2 = Rc::clone(&price);
    let share_stations = Rc::strong_count(&price);
    share_stations
}

fn main() {
    println!("stations sharing price list: {}", shared_stations(4500));
}
