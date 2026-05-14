#[path = "../src/bin/c34_exercise.rs"]
#[allow(dead_code)]
mod c34_exercise;

use c34_exercise::{validate_booking, BookingError};

#[test]
fn valid_booking() {
    assert_eq!(validate_booking(false, false), Ok("Booking confirmed".to_string()));
}

#[test]
fn slot_taken() {
    assert_eq!(validate_booking(true, false), Err(BookingError::SlotTaken));
}

#[test]
fn technician_off() {
    assert_eq!(validate_booking(false, true), Err(BookingError::TechnicianOff));
}

#[test]
fn slot_checked_first() {
    assert_eq!(validate_booking(true, true), Err(BookingError::SlotTaken));
}
