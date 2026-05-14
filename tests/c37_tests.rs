#[path = "../src/bin/c37_exercise.rs"]
#[allow(dead_code)]
mod c37_exercise;

use c37_exercise::{booking_error_message, BookingError};

#[test]
fn thiserror_slot_taken() {
    assert_eq!(booking_error_message(&BookingError::SlotTaken), "that time slot is already taken");
}

#[test]
fn thiserror_technician_off() {
    assert_eq!(booking_error_message(&BookingError::TechnicianOff), "technician is off today");
}

#[test]
fn is_std_error() {
    let e: Box<dyn std::error::Error> = Box::new(BookingError::SlotTaken);
    assert_eq!(format!("{e}"), "that time slot is already taken");
}
