#[path = "../src/bin/c53_exercise.rs"]
#[allow(dead_code)]
mod c53_exercise;

use c53_exercise::{format_booking, Args};

#[test]
fn formats_booking() {
    let args = Args {
        technician: "Mai".to_string(),
        service: "Gel Manicure".to_string(),
        price: 4500,
    };
    assert_eq!(format_booking(&args), "Booked: Gel Manicure with Mai for 4500 cents");
}

#[test]
fn formats_another() {
    let args = Args {
        technician: "Linh".to_string(),
        service: "Pedicure".to_string(),
        price: 3500,
    };
    assert_eq!(format_booking(&args), "Booked: Pedicure with Linh for 3500 cents");
}
