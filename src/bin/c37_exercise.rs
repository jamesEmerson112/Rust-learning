use thiserror::Error;

// TODO: Replace #[derive(Debug)] with #[derive(Debug, Error)]
// and add #[error("...")] messages to each variant:
// SlotTaken => "that time slot is already taken"
// TechnicianOff => "technician is off today"
#[derive(Debug, Error)]
pub enum BookingError {
    #[error("")]
    SlotTaken,
    #[error("")]
    TechnicianOff,
}

pub fn booking_error_message(err: &BookingError) -> String {
    format!("{err}")
}

fn main() {
    println!("{:?}", BookingError::SlotTaken);
}
