#[path = "../src/bin/c35_exercise.rs"]
#[allow(dead_code)]
mod c35_exercise;

use c35_exercise::{error_message, BookingError};

#[test]
fn display_slot_taken() {
    assert_eq!(error_message(&BookingError::SlotTaken), "that time slot is already taken");
}

#[test]
fn display_technician_off() {
    assert_eq!(error_message(&BookingError::TechnicianOff), "technician is off today");
}
