pub trait Service {
    fn price(&self) -> u32;
}

pub struct Manicure;
pub struct Pedicure;

impl Service for Manicure {
    fn price(&self) -> u32 {
        // TODO: A manicure costs 4500 cents.
        0
    }
}

impl Service for Pedicure {
    fn price(&self) -> u32 {
        // TODO: A pedicure costs 6000 cents.
        0
    }
}

pub fn total_price(services: &[Box<dyn Service>]) -> u32 {
    // TODO: Sum the price() of every service. Each element is a different
    // concrete type behind `dyn Service` — the call dispatches at runtime.
    let _ = services;
    0
}

fn main() {
    let booked: Vec<Box<dyn Service>> = vec![Box::new(Manicure), Box::new(Pedicure)];
    println!("Total: {} cents", total_price(&booked));
}
