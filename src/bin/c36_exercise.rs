use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum BookingError {
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

// TODO: Add this one line to wire BookingError into std::error::Error:
//   impl Error for BookingError {}
// It works because BookingError already has Debug + Display.
impl Error for BookingError {}

pub fn into_boxed_error(err: BookingError) -> Box<dyn Error> {
    Box::new(err)
}

fn main() {
    let e = into_boxed_error(BookingError::SlotTaken);
    println!("Error: {e}");
}
