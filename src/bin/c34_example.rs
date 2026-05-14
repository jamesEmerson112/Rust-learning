#[derive(Debug, PartialEq)]
enum BookingError {
    SlotTaken,
    TechnicianOff,
}

fn validate_booking(slot_taken: bool, tech_off: bool) -> Result<String, BookingError> {
    if slot_taken {
        return Err(BookingError::SlotTaken);
    }
    if tech_off {
        return Err(BookingError::TechnicianOff);
    }
    Ok("Booking confirmed".to_string())
}

fn main() {
    println!("{:?}", validate_booking(false, false));
    println!("{:?}", validate_booking(true, false));
    println!("{:?}", validate_booking(false, true));
}
