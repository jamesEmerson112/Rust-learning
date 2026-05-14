pub async fn check_availability(slot: &str) -> bool {
    // TODO: Return false if slot == "10:00" (taken), true otherwise.
    let _ = slot;
    true
}

#[tokio::main]
async fn main() {
    let avail = check_availability("10:00").await;
    println!("10:00 available? {avail}");
}
