#[path = "../src/bin/c36_exercise.rs"]
#[allow(dead_code)]
mod c36_exercise;

use c36_exercise::{into_boxed_error, BookingError};

#[test]
fn boxed_error_displays() {
    let e = into_boxed_error(BookingError::SlotTaken);
    assert_eq!(format!("{e}"), "that time slot is already taken");
}

#[test]
fn boxed_error_is_error_trait() {
    let e = into_boxed_error(BookingError::TechnicianOff);
    let _: &dyn std::error::Error = &*e;
    assert_eq!(format!("{e}"), "technician is off today");
}
