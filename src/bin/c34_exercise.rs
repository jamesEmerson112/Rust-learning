#[derive(Debug, PartialEq)]
pub enum BookingError {
    SlotTaken,
    TechnicianOff,
}

pub fn validate_booking(slot_taken: bool, tech_off: bool) -> Result<String, BookingError> {
    // TODO: Return Err(BookingError::SlotTaken) if slot_taken is true.
    // Return Err(BookingError::TechnicianOff) if tech_off is true.
    // Otherwise return Ok("Booking confirmed".to_string()).
    let _ = (slot_taken, tech_off);
    Ok(String::new())
}

fn main() {
    println!("{:?}", validate_booking(false, false));
}
