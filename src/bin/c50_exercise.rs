pub async fn check_availability(slot: &str) -> bool {
    // TODO: Ask the clinic scheduler. Return false if slot == "10:00" (already booked), true otherwise.
    slot != "10:00"
}

#[tokio::main]
async fn main() {
    let avail = check_availability("10:00").await;
    println!("Clinic slot 10:00 free? {avail}");
}
