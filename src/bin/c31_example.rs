use std::rc::Rc;

fn main() {
    let price_list = Rc::new(4500);
    let station_a = Rc::clone(&price_list);
    let station_b = Rc::clone(&price_list);

    println!("Station A sees: {} cents", station_a);
    println!("Station B sees: {} cents", station_b);
    println!("Shared owners: {}", Rc::strong_count(&price_list));
}
