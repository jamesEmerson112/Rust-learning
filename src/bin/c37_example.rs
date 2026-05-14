use thiserror::Error;

#[derive(Debug, Error)]
enum BookingError {
    #[error("that time slot is already taken")]
    SlotTaken,
    #[error("technician is off today")]
    TechnicianOff,
}

fn validate(slot_taken: bool) -> Result<(), BookingError> {
    if slot_taken {
        return Err(BookingError::SlotTaken);
    }
    Ok(())
}

fn main() {
    match validate(true) {
        Ok(()) => println!("Booked!"),
        Err(e) => println!("Error: {e}"),
    }
}
