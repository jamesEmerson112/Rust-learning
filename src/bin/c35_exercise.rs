use std::fmt;

#[derive(Debug)]
pub enum BookingError {
    SlotTaken,
    TechnicianOff,
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Match on self and write a human-readable message for each variant.
        // SlotTaken => "that time slot is already taken"
        // TechnicianOff => "technician is off today"
        match self {
            BookingError::SlotTaken => write!(f, "Slot appointment is taken"),
            BookingError::TechnicianOff => write!(f, "The technician is off"),
        }
    }
}

pub fn error_message(err: &BookingError) -> String {
    format!("{err}")
}

fn main() {
    println!("{}", BookingError::SlotTaken);
    println!("{}", BookingError::TechnicianOff);

}
