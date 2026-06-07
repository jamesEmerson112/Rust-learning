use std::fmt;

#[derive(Debug)]
enum BookingError {
    SlotTaken,
    TechnicianOff,
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookingError::SlotTaken => write!(f, "that time slot is already taken"),
            BookingError::TechnicianOff => write!(f, "technician is off today"),
        }
    }
}

fn main() {
    let err: BookingError = BookingError::SlotTaken;
    println!("Error: {err}");
    println!("Debug: {err:?}");
}
