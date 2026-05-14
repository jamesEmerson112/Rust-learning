use std::fmt;
use std::error::Error;

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

impl Error for BookingError {}

fn try_book() -> Result<String, Box<dyn Error>> {
    let err: BookingError = BookingError::SlotTaken;
    Err(Box::new(err))
}

fn main() {
    match try_book() {
        Ok(msg) => println!("{msg}"),
        Err(e) => println!("Error: {e}"),
    }
}
