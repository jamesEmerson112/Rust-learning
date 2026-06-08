#[path = "../src/bin/c57_exercise.rs"]
#[allow(dead_code)]
mod c57_exercise;

use c57_exercise::{Manicure, Pedicure, Service, total_price};

#[test]
fn individual_prices() {
    assert_eq!(Manicure.price(), 4500);
    assert_eq!(Pedicure.price(), 6000);
}

#[test]
fn mixed_services_sum() {
    let booked: Vec<Box<dyn Service>> = vec![Box::new(Manicure), Box::new(Pedicure)];
    assert_eq!(total_price(&booked), 10500);
}

#[test]
fn empty_is_zero() {
    let booked: Vec<Box<dyn Service>> = vec![];
    assert_eq!(total_price(&booked), 0);
}
