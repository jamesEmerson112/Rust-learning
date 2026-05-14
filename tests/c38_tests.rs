#[path = "../src/bin/c38_exercise.rs"]
#[allow(dead_code)]
mod c38_exercise;

use c38_exercise::process_booking;

#[test]
fn valid_booking() {
    let result = process_booking("4500", false);
    assert_eq!(result.unwrap(), "Booked at 4500 cents");
}

#[test]
fn invalid_price() {
    let result = process_booking("abc", false);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("invalid price number"));
}

#[test]
fn slot_taken() {
    let result = process_booking("4500", true);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("slot is already taken"));
}
