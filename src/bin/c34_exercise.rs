#[derive(Debug, PartialEq)]
pub enum BookingError {
    SlotTaken,
    TechnicianOff,
}

pub fn validate_booking(slot_taken: bool, tech_off: bool) -> Result<String, BookingError> {
    // TODO: Return Err(BookingError::SlotTaken) if slot_taken is true.
    // Return Err(BookingError::TechnicianOff) if tech_off is true.
    // Otherwise return Ok("Booking confirmed".to_string()).
    if slot_taken {
        return Err(BookingError::SlotTaken);
    }
    if tech_off
    {
        return Err(BookingError::TechnicianOff);
    }
    Ok("Booking confirmed".to_string())
}

fn main() {
    println!("{:?}", validate_booking(false, false));
}
